use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config_gen::{handle_generate, GenerationConfig};

pub mod config_gen;
pub mod configuration;
pub mod scraper;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

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
        #[arg(short = 'm', long, value_name = "markscheme", default_value_t = true)]
        download_markscheme: bool,
        #[arg(short = 'p', long, value_name = "paper", default_value_t = true)]
        download_paper: bool,
        #[arg(
            short = 'e',
            long,
            value_name = "examiners-report",
            default_value_t = false
        )]
        download_examiners_report: bool,
        #[arg(short, long, value_name = "years")]
        years: Option<Vec<String>>,
        #[arg(short, long, value_name = "subjects")]
        subjects: Option<Vec<String>>
    },

    #[command(about = "Download the files specified in the configuration file.")]
    Download {
        #[arg(short, long, value_name = "config", default_value = "config.toml")]
        config: String,
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
            config: _,
            output: _ ,
        } => {
            debug!("Selected Download subcommand.");
            todo!("Download subcommand not implemented yet.");
        }
        Subs::GenerateConfig {
            output,
            download_markscheme,
            download_paper,
            download_examiners_report,
            years,
            subjects,
        } => {
            debug!("Selected GenerateConfig subcommand.");
            handle_generate(GenerationConfig::new(
                output,
                download_markscheme,
                download_paper,
                download_examiners_report,
                years,
                subjects,
                args.threads,
            ));
        }
    }
}
