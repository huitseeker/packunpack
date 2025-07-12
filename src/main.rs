use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod lsf;
mod lsx;
mod resource;
mod compression;

use resource::Resource;

#[derive(Parser)]
#[command(name = "larian-convert")]
#[command(about = "Convert between LSF and LSX file formats")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert LSF (binary) to LSX (XML)
    ToXml {
        /// Input LSF file
        input: PathBuf,
        /// Output LSX file
        output: PathBuf,
    },
    /// Convert LSX (XML) to LSF (binary)
    ToBinary {
        /// Input LSX file
        input: PathBuf,
        /// Output LSF file
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ToXml { input, output } => {
            println!("Converting {} to {}", input.display(), output.display());
            let resource = lsf::read_lsf(&input)?;
            lsx::write_lsx(&resource, &output)?;
            println!("Conversion completed successfully");
        }
        Commands::ToBinary { input, output } => {
            println!("Converting {} to {}", input.display(), output.display());
            let resource = lsx::read_lsx(&input)?;
            lsf::write_lsf(&resource, &output)?;
            println!("Conversion completed successfully");
        }
    }

    Ok(())
}