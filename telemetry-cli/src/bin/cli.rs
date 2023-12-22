use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

use std::path::PathBuf;
use telemetry_cli::{parse, record};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Record {
        #[clap(short, long)]
        file: PathBuf,
    },
    Parse {
        #[clap(short, long)]
        file: PathBuf,
        #[clap(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Record { file } => {
            record(file)?;
        }
        Commands::Parse { file, out } => {
            parse(file, out)?;
        }
    }

    Ok(())
}
