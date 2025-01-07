# 🐳 Test-Containers for Quick Magento Development
[![Docker Build](https://github.com/EcomDev/testcontainer-magento-data/actions/workflows/docker-images.yml/badge.svg)](https://github.com/EcomDev/testcontainer-magento-data/actions/workflows/docker-images.yml)
[![Rust Build](https://github.com/EcomDev/testcontainer-magento-data-rust/actions/workflows/rust-package.yml/badge.svg)](https://github.com/EcomDev/testcontainer-magento-data-rust/actions/workflows/rust-package.yml)

This package simplifies the process of automated testing with real database and search engine

## ✨ Features

- 📦 **Pre-configured database and search containers**: Instantly spin up containers with ready-to-use Magento data
- ⚙️ **Easy setup and use**: Use Rust package to automatically discard container after tests
- 🎯 **Blazingly Fast**: Container takes only few seconds to start, so you can focus on testing instead of waiting for db initialization

## 📋 Requirements

- **🐳 Docker**: Ensure Docker is installed and operational on your system.

## 📦 Available images

All the available Docker image version can be found in build repository [EcomDev/testcontainer-magento-data](https://github.com/EcomDev/testcontainer-magento-data?tab=readme-ov-file#-available-images) 

## Installation

Use cargo  with `--dev` flag to add it as dependency for your tests
```bash
cargo add --dev testcontainers-magento-data
```


## Examples

### MySQL container 

Run queries against Magento database build with sample data

```rust
use sqlx::MySqlPool;
use testcontainers_magento_data::runners::AsyncRunner;
use testcontainers_magento_data::core::ImageBuilder;
use testcontainers_magento_data::images::{DbConnection, DbContainer};

#[tokio::test]
async fn starts_container_with_sample_data() {
    let container = DbContainer::mariadb()
        .with_sample_data()
        .with_version("2.4.7-p2")
        .start()
        .await
        .unwrap();

    let connection = MySqlPool::connect(&container.connection_url().await.unwrap())
        .await
        .unwrap();

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM catalog_product_entity")
        .fetch_one(&connection)
        .await
        .unwrap();

    assert_eq!(total, 2040);
}
```

## 📜 License

This project is licensed under the MIT License. 

See the [LICENSE](LICENSE) file for more details.