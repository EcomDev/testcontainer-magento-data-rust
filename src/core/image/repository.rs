pub const DEFAULT_IMAGE_REPOSITORY: DefaultRepository = DefaultRepository;
pub const DEFAULT_DISTRIBUTION: &str = "magento";

pub struct DefaultRepository;

pub trait ImageRepository {
    fn image_path(&self, image: &str, distribution: &str) -> String;
}

impl<T> ImageRepository for T
where
    T: std::fmt::Display,
{
    fn image_path(&self, image: &str, _: &str) -> String {
        format!("{self}/{image}")
    }
}

impl ImageRepository for DefaultRepository {
    fn image_path(&self, image: &str, distribution: &str) -> String {
        format!("ghcr.io/ecomdev/testcontainer-{distribution}-data/{image}")
    }
}
