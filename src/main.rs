pub mod api_client;
pub mod data;
pub mod caching_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = api_client::Client::new();

    println!("Before");
    let mut caching_client = caching_client::Client::new(client).await?;
    println!("After");

    caching_client.vs_currencies().await?;
    caching_client.vs_currencies().await?;

    caching_client.coins().await?;
    caching_client.coins().await?;

    Ok(())
}