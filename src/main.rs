use std::time::Duration;

use tokio::time::sleep;

pub mod api_client;
pub mod data;
pub mod caching_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = api_client::Client::new();

    let caching_client = caching_client::Client::new(client).await?;

    let _res1 = caching_client.vs_currencies().await?;
    let res2 = caching_client.vs_currencies().await?;

    let _res3 = caching_client.coins().await?;
    let res4 = caching_client.coins().await?;

    let _res5 = caching_client.market_chart("bitcoin", "usd", 1392577232, 1422577232).await?;
    let res6 = caching_client.market_chart("bitcoin", "usd", 1392577232, 1422577232).await?;

    println!("\nEverything is done!\n");
    sleep(Duration::from_secs(1)).await;

    println!("some vs_currencies = {:?}\n", res2.into_iter().take(10).collect::<Vec<_>>());
    println!("some coins = {:?}\n", res4.into_iter().take(10).collect::<Vec<_>>());
    println!("some market chart data prices = {:?}\n", res6.raw.prices.into_iter().take(10).collect::<Vec<_>>());

    Ok(())
}