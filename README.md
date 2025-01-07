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
cargo add --dev 
```


## Examples

### MySQL container 

Create Latest Magento Database Build
```php
use EcomDev\TestContainers\MagentoData\DbContainerBuilder;

$container = DbContainerBuilder::mysql()
    ->build();
```

Create Latest Magento Database Build with sample data
```php
use EcomDev\TestContainers\MagentoData\DbContainerBuilder;

$container = DbContainerBuilder::mysql()
    ->withSampleData()
    ->build();
```

Create 2.4.7-p2 with sample data and fetch number of products
```php
use EcomDev\TestContainers\MagentoData\DbContainerBuilder;
use PDO;

$container = DbContainerBuilder::mysql()
    ->withMagentoVersion('2.4.7-p2')
    ->withSampleData()
    ->build();

$connectionSettings = $container->getConnectionSettings();
$connection = new PDO(
    $connectionSettings->dsn(),
    $connectionSettings->user,
    $connectionSettings->password
);

$result = $connection->query('SELECT COUNT(*) FROM catalog_product_entity');
// Outputs 2040
echo $result->fetch(PDO::FETCH_COLUMN);
```

### MariaDB container
Everything the same as for MySQL container, just a different builder method

```php
use EcomDev\TestContainers\MagentoData\DbContainerBuilder;

$container = DbContainerBuilder::mariadb()
    ->withMagentoVersion('2.4.7-p2')
    ->withSampleData()
    ->build();
```

## OpenSearch container

For OpenSearch container there is a different builder and container, that allows building base url for http connection

Here is a small example

```php
use EcomDev\TestContainers\MagentoData\OpenSearchContainerBuilder;
use GuzzleHttp\Client;

$container = OpenSearchContainerBuilder::new()
            ->withSampleData()
            ->build();

$client = new Client([
    'base_uri' => $container->getBaseUrl()
]);

$result = json_decode(
    $client->get('magento2_product_1/_count')->getBody()->getContents(),
    true
);

// Outputs 181
echo $result['count'];
```

## 📜 License

This project is licensed under the MIT License. 

See the [LICENSE](LICENSE) file for more details.