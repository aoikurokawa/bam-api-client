use bam_api_client::{client::BamApiClient, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::custom("base_url");
    let client = BamApiClient::new(config);

    let validators = client
        .get_validators()
        .await
        .expect("Failed to get validators");

    println!("Validators: {:?}", validators);
}
