use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de::Visitor, Deserialize};
use serde_repr::Deserialize_repr;
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerInformation {
    pub username: String,
    pub country: String,
    #[serde(rename = "Game mode")]
    pub game_mode: GameMode,
    #[serde(rename = "fresh_start_account")]
    #[serde_as(as = "BoolFromInt")]
    pub fresh_start_account: bool,
    #[serde(rename = "Cb-3")]
    #[serde_as(as = "BoolFromInt")]
    pub combat_level_3: bool,
    #[serde_as(as = "BoolFromInt")]
    pub f2p: bool,
    #[serde_as(as = "BoolFromInt")]
    pub banned: bool,
    #[serde_as(as = "BoolFromInt")]
    pub disqualified: bool,
    #[serde(rename = "Clan preference")]
    pub clan_preference: Option<u32>,
    #[serde(rename = "Last checked")]
    pub last_checked: Option<Timestamp>,
    #[serde(rename = "Last changed")]
    pub last_changed: Option<Timestamp>,
    #[serde(rename = "Last changed KC")]
    pub last_changed_kc: Option<Timestamp>,
    #[serde(rename = "Datapoint Cooldown")]
    pub datapoint_cooldown: String,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum GameMode {
    Normal = 0,
    Ironman = 1,
    UltimateIronman = 2,
    HardcoreIronman = 3,
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a timestamp in the format %Y-%m-%d %H:%M:%S")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Timestamp(DateTime::from_utc(
            NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)
            })?,
            Utc,
        )))
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimestampVisitor)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Skills {
    pub overall: u64,
    pub attack: u32,
    pub defence: u32,
    pub strength: u32,
    pub hitpoints: u32,
    pub ranged: u32,
    pub prayer: u32,
    pub magic: u32,
    pub cooking: u32,
    pub woodcutting: u32,
    pub fletching: u32,
    pub fishing: u32,
    pub firemaking: u32,
    pub crafting: u32,
    pub smithing: u32,
    pub mining: u32,
    pub herblore: u32,
    pub agility: u32,
    pub thieving: u32,
    pub slayer: u32,
    pub farming: u32,
    pub runecraft: u32,
    pub hunter: u32,
    pub construction: u32,
    pub ehp: f32,
}
