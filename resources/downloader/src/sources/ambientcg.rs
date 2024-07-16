use super::MaterialsSource;
use crate::utils::{download_and_unzip, get, log_err};
use anyhow::{ensure, Context as _, Result};
use serde::Deserialize;
use std::path::Path;
use tracing::debug;
use url::Url;

/// ambientcg.com
#[derive(Debug, clap::Parser)]
pub struct AmbientCg {
    #[clap(long, default_value_t = 20)]
    pub limit: usize,
}

impl MaterialsSource for AmbientCg {
    const NAME: &'static str = "ambientCg";

    fn download(&self, target_dir: &Path) -> Result<()> {
        let materials: FullJson = get(&materials_url(self.limit)?)
            .context("fetching index")?
            .json()
            .context("parse index")?;

        debug!(num = materials.found_assets.len(), "got materials");

        fs_err::create_dir_all(target_dir).context("failed to create download dir")?;

        let mut success = true;
        for asset in materials.found_assets {
            if let Err(e) = download_asset(asset, target_dir) {
                log_err(&e.context("failed to download asset"));
                success = false;
            }
        }

        ensure!(success, "failed to download all assets");

        Ok(())
    }
}

#[tracing::instrument(level = "info", skip_all, fields(name=asset.display_name))]
fn download_asset(asset: Asset, target_dir: &Path) -> Result<()> {
    let download = asset.smallest_download()?;
    let name = &asset.display_name;

    download_and_unzip(&download.full_download_path, &asset.asset_id, target_dir)
        .with_context(|| format!("downloading {name} failed"))?;

    Ok(())
}

fn materials_url(limit: usize) -> Result<Url> {
    let mut url = Url::parse("https://ambientcg.com/api/v2/full_json")?;
    url.query_pairs_mut()
        .append_pair("include", "downloadData,displayData")
        .append_pair("type", "Material")
        .append_pair("limit", limit.to_string().as_str());
    Ok(url)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FullJson {
    found_assets: Vec<Asset>,
}

#[allow(unused)] // some fields only for debugging
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    asset_id: String,
    display_name: String,
    short_link: Url,
    download_folders: serde_json::Value,
}

impl Asset {
    fn smallest_download(&self) -> Result<Download> {
        let downloads: Vec<Download> = serde_json::from_value(
            self.download_folders["default"]["downloadFiletypeCategories"]["zip"]["downloads"]
                .clone(),
        )
        .context("failed to get downloads")?;

        downloads
            .into_iter()
            .min_by_key(|x| x.size)
            .context("no downloads")
    }
}

#[allow(unused)] // some fields only for debugging
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Download {
    full_download_path: Url,
    file_name: String,
    size: usize,
}
