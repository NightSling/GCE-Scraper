use std::{fs::File, path::PathBuf};

use crate::configuration::Configuration;


#[derive(Debug)]
pub struct DownloadConfiguration {
    pub config: Configuration,
    pub output_folder: PathBuf
}
#[derive(Debug)]
pub enum DownloadError {
    ConfigNotFound,
    ConfigParseError(std::io::Error),
    DownloadFolderCannotBeCreated
}

impl DownloadConfiguration {
    pub fn new(config: PathBuf, output_folder: PathBuf) -> Result<DownloadConfiguration, DownloadError> {
        // Make sure config exists
        if !config.exists() {
            return Err(DownloadError::ConfigNotFound);
        }
        let file = match File::open(&config) {
            Ok(file) => file,
            Err(_) => return Err(DownloadError::ConfigNotFound)
        };
        
        // Make sure output folder exists, if not create it
        if !output_folder.exists() {
            match std::fs::create_dir_all(&output_folder) {
                Ok(_) => {
                    return Ok(DownloadConfiguration {
                        config: match Configuration::try_from(file) {
                            Ok(config) => config,
                            Err(e) => return Err(DownloadError::ConfigParseError(e))
                        },
                        output_folder
                    });
                }
                Err(_) => return Err(DownloadError::DownloadFolderCannotBeCreated)
            }
        }
        Ok(DownloadConfiguration {
            config: match Configuration::try_from(file) {
                Ok(config) => config,
                Err(e) => return Err(DownloadError::ConfigParseError(e))
            },
            output_folder
        })
    }
    
}

pub fn handle_download(config: DownloadConfiguration) {
    todo!("Download subcommand not implemented yet.");
}