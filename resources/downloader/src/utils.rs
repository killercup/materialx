use std::{fmt::Write, io::Cursor, path::Path, time::Duration};

use anyhow::{Context as _, Result};
use tracing::info;
use url::Url;
use zip::ZipArchive;

#[tracing::instrument(level = "debug")]
pub fn get(url: &Url) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .timeout(Duration::from_secs(10))
        .build()?;
    let response = client
        .get(url.clone())
        .header(reqwest::header::ACCEPT, "application/json,*/*")
        .send()
        .with_context(|| format!("could not download {url}"))?;
    Ok(response)
}

#[tracing::instrument(level = "debug", skip_all, fields(file_name))]
pub fn download_and_unzip(url: &Url, file_name: &str, target_dir: &Path) -> Result<()> {
    let path = target_dir.join(file_name);
    if path.exists() {
        info!("target already exists, skipping");
        return Ok(());
    }

    let bytes = get(url)
        .context("failed to download zipped material")?
        .bytes()?;
    let res = Cursor::new(bytes.to_vec());
    ZipArchive::new(res)?
        .extract(&path)
        .with_context(|| format!("failed to unzip downloaded file to {path:?}"))?;
    info!(?path, "downloaded");

    Ok(())
}

pub fn log_err(error: &anyhow::Error) {
    let mut source = format!("{error}");
    let mut e = error.source();
    while let Some(inner) = e {
        let _ = write!(&mut source, " > {inner}");
        e = inner.source();
    }
    tracing::error!("{source}");
}
