use testcontainers::runners::AsyncRunner;
use testcontainers_magento_data::core::ImageBuilder;
use testcontainers_magento_data::images::OpenSearchContainer;

#[tokio::test]
async fn starts_container_with_sample_data() {
    OpenSearchContainer::new()
        .with_sample_data()
        .with_version("2.4.7-p3")
        .start()
        .await
        .unwrap();
}

#[tokio::test]
async fn starts_container_with_sample_data_from_mageos() {
    OpenSearchContainer::new()
        .with_sample_data()
        .with_distribution("mageos")
        .with_version("3.4.0")
        .start()
        .await
        .unwrap();
}
