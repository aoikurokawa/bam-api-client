# BAM API Client

A Rust client library for interacting with the BAM API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bam-api-client = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use bam_api_client::{client::BamApiClient, config::Config};

#[tokio::main]
async fn main() {
    // Create a client with default configuration
    let config = Config::custom("https://api.example.com");
    let client = BamApiClient::new(config);

    // Fetch all validators
    match client.get_validators().await {
        Ok(validators) => {
            println!("Found {} validators", validators.len());
            for validator in validators {
                println!("Validator: {:?}", validator);
            }
        }
        Err(e) => eprintln!("Error fetching validators: {}", e),
    }
}
```

## Configuration

### Using Custom Configuration

```rust
use std::time::Duration;

use bam_api_client::config::Config;

// Create a custom configuration
let config = Config::custom("https://api.example.com")
    .with_timeout(Duration::from_secs(30))
    .with_retry(true)
    .with_max_retries(5);

let client = BamApiClient::new(config);
```

### Configuration Options

The `Config` struct supports the following options:

- **base_url** - The base URL of the BAM API endpoint
- **timeout** - Request timeout duration (default: 30 seconds)
- **retry_enabled** - Enable/disable automatic retries (default: true)
- **max_retries** - Maximum number of retry attempts (default: 3)

## License

[LICENSE](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For issues and questions, please open an issue on the GitHub repository.
