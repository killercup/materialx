pub mod ambientcg;
pub mod matlib;

pub trait MaterialsSource {
    const NAME: &'static str;

    fn download(&self, target_dir: &std::path::Path) -> anyhow::Result<()>;
}
