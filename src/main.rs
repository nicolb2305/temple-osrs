#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
pub mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let client = api::Client::new();
    // dbg!(client.player_information("Posemann").await?);
    dbg!(client
        .player_datapoints("Posemann", 1_000_000_000)
        .await?
        .pop_first());
    Ok(())
}
