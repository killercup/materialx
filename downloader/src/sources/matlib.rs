use super::{MaterialsSource, Metadata};
use crate::utils::{download_and_unzip, get, log_err};
use anyhow::{ensure, Context as _, Result};
use serde::Deserialize;
use std::path::Path;
use tracing::debug;
use url::Url;

/// matlib.gpuopen.com
#[derive(Debug, clap::Parser)]
pub struct MatLib {
    #[clap(long, default_values = ["Cobblestone", "Metal", "Marble", "SciFi"])]
    pub categories: Vec<String>,
}

impl MaterialsSource for MatLib {
    const NAME: &'static str = "MatLib";

    fn download(&self, target_dir: &Path) -> Result<()> {
        for category in &self.categories {
            download_materials(category, target_dir)
                .with_context(|| format!("downloading materials for {category} failed"))?;
        }
        Ok(())
    }
}

#[tracing::instrument(level = "info", skip_all, fields(%category))]
fn download_materials(category: &str, target_dir: &Path) -> Result<()> {
    let target_dir = target_dir.join(category);
    let materials: Response<Material> = get(&materials_url(category)?)?
        .json()
        .context("failed to fetch materials list")?;

    ensure!(materials.count > 0, "no materials found in category");
    debug!(num = materials.count, "got materials");

    fs_err::create_dir_all(&target_dir).context("failed to create download dir")?;
    let mut success = true;
    for material in materials.results {
        if let Err(e) = download_asset(material, &target_dir) {
            log_err(&e.context("failed to download asset"));
            success = false;
        }
    }

    ensure!(success, "failed to download all assets");

    Ok(())
}

#[tracing::instrument(level = "info", skip_all, fields(name=material.title))]
fn download_asset(material: Material, target_dir: &Path) -> Result<()> {
    let file_name = &material.title;

    ensure!(
        !material.packages.is_empty(),
        "material {file_name} has no packages"
    );
    let mut packages = material
        .packages
        .iter()
        .map(|id| get_package(id).with_context(|| format!("failed to fetch package {id}")))
        .collect::<Result<Vec<Package>>>()
        .context("failed to fetch packages")?;

    packages.sort_by_key(|p| p.label.clone());
    let smallest_package = packages.first().unwrap();
    debug!(package=%smallest_package.id, size=%smallest_package.size, "chose package");

    let meta = Metadata {
        source: MatLib::NAME.to_string(),
        name: material.title.clone(),
        id: material.id.clone(),
        url: public_url(&material.id)?.to_string(),
        preview_image: None,
    };

    download_and_unzip(
        &download_url(&smallest_package.id)?,
        file_name,
        target_dir,
        &meta,
    )
    .with_context(|| format!("downloading {file_name} failed"))?;

    Ok(())
}

fn get_package(package_id: &str) -> Result<Package> {
    Ok(get(&package_url(package_id)?)?.json()?)
}

fn materials_url(category: &str) -> Result<Url> {
    let mut url = Url::parse("https://api.matlib.gpuopen.com/api/materials/")?;
    url.query_pairs_mut()
        .append_pair("category", category)
        .append_pair("license", "MIT Public Domain")
        .append_pair("limit", "200")
        .append_pair("offset", "0")
        .append_pair("ordering", "-published_date")
        .append_pair("status", "Published")
        .append_pair("updateKey", "1");
    Ok(url)
}

fn package_url(package_id: &str) -> Result<Url> {
    let url = Url::parse(&format!(
        "https://api.matlib.gpuopen.com/api/packages/{package_id}/"
    ))?;
    Ok(url)
}

pub fn download_url(package_id: &str) -> Result<Url> {
    let mut url = Url::parse("https://api.matlib.gpuopen.com/api/packages")?;
    url.path_segments_mut()
        .expect("valid url")
        .push(package_id)
        .push("download");
    Ok(url)
}

pub fn public_url(package_id: &str) -> Result<Url> {
    let mut url = Url::parse("https://matlib.gpuopen.com/main/materials/all")?;
    url.query_pairs_mut().append_pair("material", package_id);
    Ok(url)
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    count: usize,
    results: Vec<T>,
}

#[allow(unused)] // some fields only for debugging
#[derive(Debug, Deserialize)]
struct Material {
    id: String,
    title: String,
    packages: Vec<String>,
    mtlx_filename: String,
    mtlx_material_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Package {
    pub id: String,
    pub file_url: String,
    pub size: String,
    pub file: String,
    pub label: String,
}
