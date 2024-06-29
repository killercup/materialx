use anyhow::Context as _;
use matlib_downloader::download_materials;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // chose a category from https://matlib.gpuopen.com/main/materials/all
    let categories = ["Base Materials", "Metal"];
    let target_dir = PathBuf::from("../materials");
    for category in categories {
        download_materials(category, &target_dir)
            .with_context(|| format!("downloading materials for {category} failed"))?;
    }
    Ok(())
}
