use crate::core::{DEFAULT_DISTRIBUTION, DEFAULT_IMAGE_REPOSITORY, ImageRepository};

#[derive(Debug, PartialEq)]
pub struct ImageName {
    name: String,
    path: String,
}

impl ImageName {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let path = DEFAULT_IMAGE_REPOSITORY.image_path(&name, DEFAULT_DISTRIBUTION);
        Self { name, path }
    }

    pub fn with_distribution(self, distribution: &str) -> Self {
        let path = DEFAULT_IMAGE_REPOSITORY.image_path(&self.name, distribution.as_ref());
        Self { path, ..self }
    }

    pub fn with_repository(self, repository: impl ImageRepository) -> Self {
        let path = repository.image_path(&self.name, DEFAULT_DISTRIBUTION);
        Self { path, ..self }
    }
}

impl AsRef<str> for ImageName {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_name_from_default_repository() {
        ImageName::new("mysql");

        assert_eq!(
            ImageName::new("mysql").as_ref(),
            "ghcr.io/ecomdev/testcontainer-magento-data/mysql"
        );
    }

    #[test]
    fn allows_to_override_repository_of_the_image() {
        assert_eq!(
            ImageName::new("mariadb")
                .with_repository("docker.io/library")
                .as_ref(),
            "docker.io/library/mariadb"
        );
    }
}
