use std::path::PathBuf;

use anyhow::{Context as _, Result};
use clap::Parser;
use materials_downloader::sources::{
    ambientcg::{self, AmbientCg},
    matlib::{self, MatLib},
    MaterialsSource as _,
};
use tracing::info;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    download_dir: Option<PathBuf>,
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
    let target_dir = if let Some(dir) = args.download_dir {
        dir
    } else {
        workspace_dir()
            .context("can't find workspace dir")?
            .join("assets/materials")
    };
    info!("Downloading materials to `{:?}`", target_dir);
    match args.source {
        Source::AmbientCg(source) => source.download(&target_dir.join(AmbientCg::NAME)),
        Source::MatLib(source) => source.download(&target_dir.join(MatLib::NAME)),
    }
    .context("failed to download materials")?;

    Ok(())
}

fn workspace_dir() -> Result<PathBuf> {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .context("Can't run `cargo locate-project`")?
        .stdout;
    let output = std::str::from_utf8(&output)
        .context("path not utf8")?
        .trim();
    let cargo_path = std::path::Path::new(output);
    Ok(cargo_path
        .parent()
        .context("can't get parent dir of `Cargo.toml`")?
        .to_path_buf())
}
