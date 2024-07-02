pub mod sources;
pub(crate) mod utils;

pub trait MaterialsSource {
    const NAME: &'static str;

    fn download(&self, target_dir: &std::path::Path) -> anyhow::Result<()>;
}
