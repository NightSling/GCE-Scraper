use std::{fs::File, io::Write, path::PathBuf, vec};

use futures::StreamExt;

use crate::{
    configuration::{Configuration, PaperType, RawPaper, Season, SyllabusCode, YearConfiguration, SYLLABUS_CODES},
    scraper::{get_all_papers, get_all_years, PaperRequest},
};
#[derive(Debug)]
pub struct PaperGenerationConfig {
    pub papers: Vec<PaperType>,
    pub years: Option<Vec<String>>,
    pub subjects: Option<Vec<String>>,
    pub seasons: Option<Vec<Season>>,
}

#[derive(Debug)]
pub struct GenerationConfig {
    output: File,
    paper_generation_config: PaperGenerationConfig,
    threads: u8,
}

impl GenerationConfig {
    pub fn new(
        output: PathBuf,
        paper_generation_config: PaperGenerationConfig,
        threads: u8,
    ) -> Self {
        let output = match File::create(output) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create configuration file: {}", e);
                std::process::exit(1);
            }
        };
        Self {
            output,
            paper_generation_config,
            threads,
        }
    }
}

pub fn handle_generate(mut config: GenerationConfig) {
    info!("Generating configuration file at {:?}", config.output);
    debug!("Configuration: {:?}", config);

    // Generate the configuration file
    let mut f_config = Configuration {
        papers: config.paper_generation_config.papers.clone(),
        subjects: vec![],
    };

    let seasons = config.paper_generation_config.seasons.unwrap_or(vec![
        Season::March,
        Season::Summer,
        Season::Winter,
    ]);

    let syllabus_codes: Vec<SyllabusCode> = config
        .paper_generation_config
        .subjects
        .map(|x| {
            x.iter()
                .filter_map(|y| {
                    let code = SYLLABUS_CODES
                        .iter()
                        .find(|&code| {
                            code.name.to_lowercase().starts_with(&y.to_lowercase())
                                || code
                                    .syllabus_code
                                    .to_lowercase()
                                    .starts_with(&y.to_lowercase())
                        })
                        .cloned();
                    if code.is_none() {
                        error!("Invalid subject code: {}", y);
                        std::process::exit(1);
                    }
                    code
                })
                .collect()
        })
        .unwrap_or(SYLLABUS_CODES.clone());

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads as usize)
        .enable_all()
        .build();

    let rt = match rt {
        Ok(rt) => rt,
        Err(e) => {
            error!("Failed to create tokio runtime: {}", e);
            std::process::exit(1);
        }
    };

    let raw_papers = syllabus_codes.iter().map(|x| RawPaper {
        year: config
            .paper_generation_config
            .years
            .clone()
            .unwrap_or_default(),
        syllabus_code: x.clone(),
    });
    let raw_papers = rt.block_on(async {
        futures::stream::iter(raw_papers)
            .map(|paper| async move {
                if !paper.year.is_empty() {
                    return Some(paper);
                }
                let years = get_all_years(&paper.syllabus_code).await;
                match years {
                    Ok(years) => Some(RawPaper {
                        year: years,
                        syllabus_code: paper.syllabus_code,
                    }),
                    Err(e) => {
                        error!(
                            "Failed to fetch years for {}: {:?}",
                            paper.syllabus_code.name, e
                        );
                        None
                    }
                }
            })
            .buffer_unordered(config.threads as usize)
            .collect::<Vec<_>>()
            .await
    });
    let raw_papers = raw_papers.into_iter().flatten().collect::<Vec<_>>();
    if raw_papers.is_empty() {
        error!("No papers found.");
        std::process::exit(1);
    }

    let paper_request = raw_papers.iter().flat_map(|paper| {
        paper.year.iter().map(|year| {
            PaperRequest {
                syllabus: paper.syllabus_code.clone(),
                year: year.clone(),
                seasons: seasons.clone(),
                papers: config.paper_generation_config.papers.clone(),
            }
        })
    }).collect::<Vec<_>>();

    // async block
    let papers = rt.block_on(async {
        futures::stream::iter(paper_request)
            .map(|request| async move {
                let papers = get_all_papers(&request).await;
                if papers.is_empty() {
                    error!("No papers found for {:?}", request);
                }
                YearConfiguration {
                    papers,
                    syllabus_code: request.syllabus.clone()
                }
            })
            .buffer_unordered(config.threads as usize)
            .collect::<Vec<_>>()
            .await
    }).into_iter().collect::<Vec<_>>();
    

    f_config.subjects = papers;
    
    let toml_config = toml::to_string(&f_config).unwrap();

    // as_bytes() is cheap.
    debug!(
        "Writing {} bytes to configuration file.",
        toml_config.as_bytes().len()
    );
    match config.output.write_all(toml_config.as_bytes()) {
        Ok(_) => {
            info!("Configuration file generated successfully.");
        }
        Err(e) => {
            error!("Failed to write to configuration file: {}", e);
            std::process::exit(1);
        }
    }
}
