use std::path::PathBuf;

use clap::{Parser, Subcommand};
use gce_scraper::{config_gen::{handle_generate, GenerationConfig, PaperGenerationConfig}, configuration::{PaperType, Season}, download::{handle_download, DownloadConfiguration}};
use log::debug;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[arg(
        short,
        long,
        value_name = "threads",
        default_value = "4",
        long_help = "Number of threads to use for I/O operations."
    )]
    threads: u8,

    #[command(subcommand)]
    generate: Subs,
}

#[derive(Subcommand, Debug)]
enum Subs {
    #[command(about = "Generate a configuration file consisting of all download values.")]
    GenerateConfig {
        #[arg(short, long, value_name = "output", default_value = "config.toml")]
        output: PathBuf,
        #[arg(short = 'p', long, value_name = "paper", value_delimiter=',', default_value="qp,ms,er")]
        papers: Vec<PaperType>,
        #[arg(short, long, value_name = "years", value_delimiter=',')]
        years: Option<Vec<String>>,
        #[arg(short, long, value_name = "subjects", value_delimiter=',')]
        subjects: Option<Vec<String>>,
        #[arg(long, value_name = "seasons", value_delimiter=',' , default_value = "winter,summer")]
        seasons: Option<Vec<Season>>
    },

    #[command(about = "Download the files specified in the configuration file.")]
    Download {
        #[arg(short, long, value_name = "config", default_value = "config.toml")]
        config: PathBuf,
        #[arg(
            short,
            long,
            value_name = "output-folder",
            default_value = "Past Papers",
            long_help = "Name of the directory to store in/create."
        )]
        output: PathBuf
    },
}
fn main() {
    let args = Args::parse();
    // Check verbosity flag and set RUST_LOG env variable
    match args.verbose.is_present() {
        true => {
            let level = args.verbose.log_level().unwrap_or(log::Level::Info);
            std::env::set_var("RUST_LOG", level.to_string());
        }
        false => {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    // Initialize logger
    pretty_env_logger::init();
    debug!(
        "Logger started with level: {}",
        std::env::var("RUST_LOG").unwrap()
    );

    // Handle subcommands
    match args.generate {
        Subs::Download {
            config,
            output,
        } => {
            debug!("Selected Download subcommand.");
            handle_download(match DownloadConfiguration::new(config, output, args.threads) {
                Ok(config) => config,
                Err(e) => {
                    match e {
                        gce_scraper::download::DownloadError::ConfigNotFound => {
                            log::error!("Configuration file not found.");
                        },
                        gce_scraper::download::DownloadError::DownloadFolderCannotBeCreated => {
                            log::error!("Output folder cannot be created.");
                        },
                        gce_scraper::download::DownloadError::ConfigParseError(e) => {
                            log::error!("Error parsing configuration file: {}", e);
                        }
                    }
                    std::process::exit(1);
                }
                
            });
        }
        Subs::GenerateConfig {
            output,
            papers,
            years,
            subjects,
            seasons,
        } => {
            debug!("Selected GenerateConfig subcommand.");
            handle_generate(GenerationConfig::new(
                output,
                PaperGenerationConfig {
                    papers,
                    years,
                    subjects,
                    seasons,
                },
                args.threads,
            ));
        }
    }
}
