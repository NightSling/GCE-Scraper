use std::{fs::File, io::Write, path::PathBuf, vec};

use crate::{
    configuration::{Configuration, YearConfiguration, SYLLABUS_CODES},
    scraper::get_all_years,
};

#[derive(Debug)]
pub struct GenerationConfig {
    output: File,
    download_markscheme: bool,
    download_paper: bool,
    download_examiners_report: bool,
    years: Option<Vec<String>>,
    subjects: Option<Vec<String>>,
    threads: u8,
}

impl GenerationConfig {
    pub fn new(
        output: PathBuf,
        download_markscheme: bool,
        download_paper: bool,
        download_examiners_report: bool,
        years: Option<Vec<String>>,
        subjects: Option<Vec<String>>,
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
            download_markscheme,
            download_paper,
            download_examiners_report,
            years,
            subjects,
            threads,
        }
    }
}

pub fn handle_generate(mut config: GenerationConfig) {
    info!("Generating configuration file at {:?}", config.output);
    debug!("Configuration: {:?}", config);

    // Generate the configuration file
    let mut f_config = Configuration {
        download_markscheme: config.download_markscheme,
        download_paper: config.download_paper,
        download_examiners_report: config.download_examiners_report,
        subjects: vec![],
    };

    f_config.subjects = match &config.subjects {
        Some(subjects) => {
            subjects
                .iter()
                .map(|subject| {
                    // match subject with syllabus code
                    let syllabus_code = &SYLLABUS_CODES.iter().find(|code| {
                        code.name
                            .to_lowercase()
                            .starts_with(&subject.to_lowercase())
                            || code.syllabus_code.to_lowercase() == subject.to_lowercase()
                    });
                    let syllabus_code = match syllabus_code {
                        Some(code) => code,
                        None => {
                            error!("Subject {} not found in syllabus codes.", subject);
                            std::process::exit(1);
                        }
                    };

                    let years = match &config.years {
                        Some(years) => years.clone(),
                        None => vec![],
                    };
                    YearConfiguration {
                        syllabus_code: (*syllabus_code).clone(),
                        years,
                    }
                })
                .collect()
        }
        None => SYLLABUS_CODES
            .iter()
            .map(|code| YearConfiguration {
                syllabus_code: code.clone(),
                years: vec![],
            })
            .collect(),
    };

    // mapping through f_config.subjects and if years is empty, getting all years
    // uses tokio runtime with multi-threaded executor
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
    let mut handles = vec![];
    for subject in f_config.subjects {
        if subject.years.is_empty() {
            let mut subject = subject.clone();
            let handle: tokio::task::JoinHandle<YearConfiguration> = rt.spawn(async move {
                let years = get_all_years(&subject.syllabus_code).await;
                let years = match years {
                    Ok(years) => {
                        debug!("Got years for subject: {:?}", years);
                        years
                    }
                    Err(e) => {
                        error!("Failed to get years for subject {}: {:?}", &subject.syllabus_code.name, e);
                        vec![]
                    }
                };
                subject.years = years;
                subject
            });
            handles.push(handle);
        }
    }
    // wait for all handles to finish
    let subjects: Vec<YearConfiguration> = rt.block_on(async {
        let mut joint_future_handles: Vec<Result<YearConfiguration, tokio::task::JoinError>> =  futures::future::join_all(handles).await;
        joint_future_handles.iter_mut().filter(|handle| 
            match handle {
                Ok(a) => !a.years.is_empty(),
                Err(e) => {
                    error!("Failed to get years for subject: {}", e);
                    false
                }
            }
        ).map(|handle| handle.as_ref().unwrap().clone()).collect()
    });

    f_config.subjects = subjects;

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
