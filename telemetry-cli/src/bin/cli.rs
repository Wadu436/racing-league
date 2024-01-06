use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;

use std::path::PathBuf;
use telemetry_cli::{initialize, parse, record};

#[derive(Parser, Debug)]
#[command(author = "Warre Dujardin", version = "0.1.0", about = "Utility for recording and parsing F1 telemetry packets", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(global = true, long, help = "Enables additional logging")]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Record UDP packets sent by F1 (the game) telemetry")]
    Record {
        #[clap(short, long)]
        file: PathBuf,
    },
    #[clap(about = "Parse a previously recorded stream of UDP packets")]
    Parse {
        #[clap(short, long)]
        file: PathBuf,
        #[clap(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::WARN
    };

    initialize(log_level)?;

    match cli.command {
        Commands::Record { file } => {
            record(file, "0.0.0.0:20777")?;
        }
        Commands::Parse { file, out } => {
            parse(file, out)?;
        }
    }

    Ok(())
}
