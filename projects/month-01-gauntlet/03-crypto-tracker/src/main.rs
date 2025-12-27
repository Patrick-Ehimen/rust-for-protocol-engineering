use serde_json::Value;
use std::env;
use std::io::{self, Write};
use anyhow::{Context, Result};
use dotenvy::dotenv;

/// CoinGecko's simple price response is dynamic: 
/// { "bitcoin": { "usd": 50000 } }
/// So we use a generic JSON Value or a Map to handle it.

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load the .env file from the project root
    dotenv().ok();

    println!("--- � CoinGecko Price Tracker � ---");

    // 2. Fetch the API Key from environment variables
    let api_key = env::var("COINGECKO_API_KEY")
        .context("COINGECKO_API_KEY not found in .env file")?;

    loop {
        print!("\nEnter a Coin ID (e.g., bitcoin, ethereum, solana) or 'exit': ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let coin_id = input.trim().to_lowercase();

        if coin_id == "exit" {
            println!("Goodbye!");
            break;
        }

        match fetch_coingecko_price(&coin_id, &api_key).await {
            Ok(price) => {
                println!("✅ {}: ${}", coin_id.to_uppercase(), price);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }

    Ok(())
}

async fn fetch_coingecko_price(coin_id: &str, api_key: &str) -> Result<f64> {
    // CoinGecko Demo API allows passing the key as a query parameter
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?vs_currencies=usd&ids={}&x_cg_demo_api_key={}",
        coin_id, api_key
    );

    let response = reqwest::get(&url)
        .await?
        .json::<Value>()
        .await?;

    // Parse the dynamic JSON: response[coin_id]["usd"]
    let price = response
        .get(coin_id)
        .and_then(|coin| coin.get("usd"))
        .and_then(|v| v.as_f64())
        .context(format!("Could not find price for '{}'. Check the ID spelling.", coin_id))?;

    Ok(price)
}
