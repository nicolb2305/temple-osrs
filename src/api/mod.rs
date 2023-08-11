use self::types::{PlayerInformation, Skills, Timestamp};
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

pub mod types;

#[derive(Deserialize, Debug)]
struct Data<T> {
    data: T,
}

#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn player_information(&self, player: &str) -> Result<PlayerInformation> {
        Ok(self
            .client
            .get("https://templeosrs.com/api/player_info.php")
            .query(&[("player", player)])
            .send()
            .await?
            .json::<Data<PlayerInformation>>()
            .await?
            .data)
    }

    pub async fn player_datapoints(
        &self,
        player: &str,
        time: u32,
    ) -> Result<BTreeMap<Timestamp, Skills>> {
        Ok(self
            .client
            .get("https://templeosrs.com/api/player_datapoints.php")
            .query(&[("player", player), ("time", &time.to_string())])
            .send()
            .await?
            .json::<Data<BTreeMap<Timestamp, Skills>>>()
            .await?
            .data)
    }
}
