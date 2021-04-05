use caching_client::CachingClient;

pub mod client;
pub mod data;
pub mod caching_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = client::Client::new();
    // let res = client.ping().await?;
    // println!("Ping result: {}", res);
    // let coins_list = client.coins_list().await?;
    // for coin in coins_list {
    //   //  println!("{:?}", coin);
    // }
    // let currencies = client.supported_vs_currencies().await?;
    // for currency in currencies {
    // //    println!("{}",currency);
    // }

    // let crypto_range = client.market_chart_range("bitcoin", "usd", 1392577232, 1422577232).await?;
    // println!("{:?}", crypto_range);

    // let prices = client.price(&vec!["bitcoin"], &vec!["usd", "eur"]).await?;
    // println!("{:?}", prices);

    let mut caching_client = CachingClient::new(client).await?;
    let currencies = caching_client.supported_vs_currencies().await?;
    println!("{:?}", currencies);

    Ok(())
}