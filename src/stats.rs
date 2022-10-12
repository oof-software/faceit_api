use crate::{client::Client, shared::unix_time_ms};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Lifetime {
    #[serde(rename = "rev")]
    matches: u16,
    created_at: Option<u64>,
    updated_at: u64,
    #[serde(flatten)]
    stats: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct Response {
    lifetime: Lifetime,
}

#[derive(Debug)]
pub struct Stats {
    pub matches: u16,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: DateTime<Local>,
    pub stats: HashMap<String, String>,
}

impl Into<Stats> for Response {
    fn into(self) -> Stats {
        let stats = self.lifetime.stats.into_iter();
        let stats = stats
            .filter_map(|(k, v)| match v {
                Value::String(str) => Some((k, str)),
                _ => None,
            })
            .collect();
        Stats {
            matches: self.lifetime.matches,
            created_at: self.lifetime.created_at.map(unix_time_ms),
            updated_at: unix_time_ms(self.lifetime.updated_at),
            stats,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/stats/v1/stats/users/{USER_ID}/games/csgo`
    /// - `https://api.faceit.com/users/v1/nicknames/{NICKNAME}`
    pub async fn stats(&self, user_id: &str) -> reqwest::Result<Stats> {
        const PREFIX: &str = "https://api.faceit.com/stats/v1/stats/users";
        const SUFFIX: &str = "games/csgo";

        let url = format!("{}/{}/{}", PREFIX, user_id, SUFFIX);

        Ok(self.get_json::<Response>(&url, &[]).await?.into())
    }
}
