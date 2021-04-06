pub mod api_client;
pub mod data;
pub mod caching_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = api_client::Client::new();

    let mut caching_client = caching_client::Client::new(client).await?;

    caching_client.supported_vs_currencies().await?;
    caching_client.supported_vs_currencies().await?;

    caching_client.coins_list().await?;
    caching_client.coins_list().await?;

    Ok(())
}