use std::path::PathBuf;

use anyhow::{Context as _, Result};
use clap::Parser;
use materials_downloader::{
    sources::{
        ambientcg::{self, AmbientCg},
        matlib,
    },
    MaterialsSource as _,
};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    download_dir: PathBuf,
    #[clap(subcommand)]
    source: Source,
}

#[derive(Debug, Parser)]
#[clap(rename_all = "lowercase")]
enum Source {
    AmbientCg(ambientcg::AmbientCg),
    MatLib(matlib::MatLib),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    match args.source {
        Source::AmbientCg(source) => source.download(&args.download_dir.join(AmbientCg::NAME)),
        Source::MatLib(source) => source.download(&args.download_dir),
    }
    .context("failed to download materials")?;

    Ok(())
}
