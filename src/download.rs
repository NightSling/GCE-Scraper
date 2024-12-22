use std::{fs::File, path::PathBuf, sync::Arc};

use futures::{stream, StreamExt};
use par_stream::ParStreamExt;

use crate::{configuration::{Configuration, Paper}, scraper::save_paper};

#[derive(Debug)]
pub struct DownloadConfiguration {
    pub config: Configuration,
    pub output_folder: PathBuf,
    pub threads: u8,
}
#[derive(Debug)]
pub enum DownloadError {
    ConfigNotFound,
    ConfigParseError(std::io::Error),
    DownloadFolderCannotBeCreated,
}

impl DownloadConfiguration {
    pub fn new(
        config: PathBuf,
        output_folder: PathBuf,
        threads: u8,
    ) -> Result<DownloadConfiguration, DownloadError> {
        // Make sure config exists
        if !config.exists() {
            return Err(DownloadError::ConfigNotFound);
        }
        let file = match File::open(&config) {
            Ok(file) => file,
            Err(_) => return Err(DownloadError::ConfigNotFound),
        };

        // Make sure output folder exists, if not create it
        if !output_folder.exists() {
            match std::fs::create_dir_all(&output_folder) {
                Ok(_) => {
                    return Ok(DownloadConfiguration {
                        config: match Configuration::try_from(file) {
                            Ok(config) => config,
                            Err(e) => return Err(DownloadError::ConfigParseError(e)),
                        },
                        output_folder,
                        threads,
                    });
                }
                Err(_) => return Err(DownloadError::DownloadFolderCannotBeCreated),
            }
        }
        Ok(DownloadConfiguration {
            config: match Configuration::try_from(file) {
                Ok(config) => config,
                Err(e) => return Err(DownloadError::ConfigParseError(e)),
            },
            threads,
            output_folder,
        })
    }
}

pub fn handle_download(config: DownloadConfiguration) {
    config.config.subjects.iter().for_each(|subject| {
        info!(
            "Downloading papers for subject: {}",
            format!(
                "{} ({})",
                subject.syllabus_code.name, subject.syllabus_code.syllabus_code
            )
        );
        // make folder for the subject
        let subject_folder = config.output_folder.join(format!(
            "{} ({})",
            subject.syllabus_code.name, subject.syllabus_code.syllabus_code
        ));
        if !subject_folder.exists() {
            match std::fs::create_dir_all(&subject_folder) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to create folder for subject: {}", e);
                    std::process::exit(1);
                }
            }
        }
        let years = &subject
            .papers
            .iter()
            .map(|paper| paper.year.clone())
            .collect::<std::collections::HashSet<String>>();

        years.iter().for_each(|year| {
            // create folder
            let year_folder = subject_folder.join(year);
            if !year_folder.exists() {
                match std::fs::create_dir_all(&year_folder) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed to create folder for year: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        });
        // cluster papers and async download it and save it.
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
        let papers = subject.papers.iter().map(|paper| {
            (paper.clone(), subject.clone(), subject_folder.clone())
        }).collect::<Vec<_>>();
        // chunk papers into threads 
        rt.block_on(async {
            stream::iter(papers)
            .par_then(None, |val| async move {
                save_paper(
                    &val.1.syllabus_code,
                    &val.0,
                    &val.2.join(&val.0.year).join(Paper::get_ref_filename(&val.0, &val.1.syllabus_code)),
                )
                .await;
            })
            .collect::<Vec<_>>()
            .await;
        });

        // rt.block_on(async {
        //     futures::stream::iter(papers)
        //         .map(|paper| async move {
        //             save_paper(
        //                 &paper.1.syllabus_code, 
        //                 &paper.0, 
        //                 &paper.2.join(&paper.0.year).join(Paper::get_ref_filename(&paper.0, &paper.1.syllabus_code))
        //             ).await
        //         })
        //         .buffer_unordered(config.threads as usize)
        //         .collect::<Vec<_>>().await
        //     }
        // );
    });
}
