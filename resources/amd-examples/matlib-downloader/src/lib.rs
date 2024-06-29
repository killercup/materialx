use anyhow::{Context as _, Result};
use reqwest::header::ACCEPT;
use serde::Deserialize;
use std::{io::Cursor, path::Path};
use url::Url;
use zip::ZipArchive;

pub fn download_materials(category: &str, target_dir: &Path) -> Result<()> {
    let target_dir = target_dir.join(category);
    let materials: Response<Material> = get(&materials_url(category)?)?
        .json()
        .context("failed to fetch materials list")?;

    eprintln!("{category}: {} materials", materials.count);

    fs_err::create_dir_all(&target_dir).context("failed to create download dir")?;
    for material in materials.results {
        let name = &material.title;
        let package_id = material
            .packages
            .first()
            .with_context(|| "material {name} has no packages")?;
        let path = target_dir.join(name);
        if path.exists() {
            eprintln!("{name} already exists, skipping");
            continue;
        }

        let bytes = get(&download_url(package_id)?)
            .context("failed to download zipped material")?
            .bytes()?;
        let res = Cursor::new(bytes.to_vec());
        ZipArchive::new(res)?
            .extract(&path)
            .with_context(|| format!("failed to unzip downloaded file to {path:?}"))?;
        eprintln!("downloaded {name} to {path:?}");
    }
    Ok(())
}

fn get(url: &Url) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        // .redirect(Policy::limited(2))
        .build()?;
    let response = client
        .get(url.clone())
        .header(ACCEPT, "application/json,*/*")
        .send()
        .with_context(|| format!("could not download {url}"))?;
    Ok(response)
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

pub fn download_url(package_id: &str) -> Result<Url> {
    let mut url = Url::parse("https://api.matlib.gpuopen.com/api/packages")?;
    url.path_segments_mut()
        .expect("valid url")
        .push(package_id)
        .push("download");
    Ok(url)
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    count: usize,
    results: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct Material {
    id: String,
    title: String,
    packages: Vec<String>,
    mtlx_filename: String,
    mtlx_material_name: String,
}
