pub mod ambientcg;
pub mod matlib;

pub trait MaterialsSource {
    const NAME: &'static str;

    fn download(&self, target_dir: &std::path::Path) -> anyhow::Result<()>;
}

#[derive(Debug, serde::Serialize)]
pub struct Metadata {
    pub source: String,
    pub name: String,
    pub id: String,
    pub url: String,
    pub preview_image: Option<String>,
}
