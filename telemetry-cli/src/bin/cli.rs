use clap::{Parser, Subcommand, ValueEnum};
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

#[derive(Copy, Clone, Debug, ValueEnum)]
enum PacketId {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
}

impl From<PacketId> for telemetry::packet::header::PacketId {
    fn from(value: PacketId) -> Self {
        match value {
            PacketId::Motion => telemetry::packet::header::PacketId::Motion,
            PacketId::Session => telemetry::packet::header::PacketId::Session,
            PacketId::LapData => telemetry::packet::header::PacketId::LapData,
            PacketId::Event => telemetry::packet::header::PacketId::Event,
            PacketId::Participants => telemetry::packet::header::PacketId::Participants,
            PacketId::CarSetups => telemetry::packet::header::PacketId::CarSetups,
            PacketId::CarTelemetry => telemetry::packet::header::PacketId::CarTelemetry,
            PacketId::CarStatus => telemetry::packet::header::PacketId::CarStatus,
            PacketId::FinalClassification => {
                telemetry::packet::header::PacketId::FinalClassification
            }
            PacketId::LobbyInfo => telemetry::packet::header::PacketId::LobbyInfo,
            PacketId::CarDamage => telemetry::packet::header::PacketId::CarDamage,
            PacketId::SessionHistory => telemetry::packet::header::PacketId::SessionHistory,
        }
    }
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
        #[clap(short = 'F', long, value_delimiter = ',')]
        filter: Option<Vec<PacketId>>,
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
        Commands::Parse { file, out, filter } => {
            parse(
                file,
                out,
                filter.map(|filters| filters.into_iter().map(Into::into).collect::<Vec<_>>()),
            )?;
        }
    }

    Ok(())
}
