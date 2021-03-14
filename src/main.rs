pub mod client;
pub mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = client::Client::new();
    let res = client.ping().await?;
    println!("Ping result: {}", res);
    let coins_list = client.coins_list().await?;
    for coin in coins_list {
        println!("{:?}", coin);
    }
    let currencies = client.simple_supported_vs_currencies().await?;
    for currency in currencies {
        println!("{}",currency);
    }

    //let cryptoRange = client.coins_id_market_chart_range("btc", "usd", 1392577232, 1392677232).await?;
    //println!("{:?}", cryptoRange);

    Ok(())
}