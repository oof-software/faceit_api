use crate::{
    client::Client,
    mapping::{MapStats, Mapping},
    shared::unix_time_ms,
};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Id {
    #[serde(rename = "matchId")]
    match_id: String,
    #[serde(rename = "playerId")]
    player_id: String,
}

#[derive(Deserialize)]
struct Match_ {
    #[serde(rename = "_id")]
    id: Id,
    created_at: Option<u64>,
    updated_at: Option<u64>,
    nickname: String,
    #[serde(rename = "playerId")]
    player_id: String,
    #[serde(rename = "teamId")]
    team_id: String,
    premade: Option<bool>,
    #[serde(rename = "bestOf")]
    best_of: Option<String>,
    #[serde(rename = "competitionId")]
    competition_id: Option<String>,
    date: u64,
    game: String,
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "matchId")]
    match_id: String,
    #[serde(rename = "matchRound")]
    match_round: Option<String>,
    played: Option<String>,
    status: String,
    elo: Option<Value>,
    #[serde(flatten)]
    stats: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct Response(Vec<Match_>);

#[derive(Debug)]
pub struct Match {
    pub match_id: String,
    pub date: DateTime<Local>,
    pub elo: Option<u16>,
    pub premade: Option<bool>,
    pub team_id: String,
    pub game_mode: String,
    pub best_of: String,
    pub played: String,
    pub status: String,
    pub game: String,
    pub stats: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Matches(pub Vec<Match>);

impl Matches {
    pub fn map_stats(&mut self, mapping: &Mapping) {
        self.0.iter_mut().for_each(|m| m.stats.map_stats(mapping))
    }
}

impl Into<Match> for Match_ {
    fn into(self) -> Match {
        let date = unix_time_ms(self.date);
        let elo = self.elo.map(|elo| match elo {
            Value::Number(num) => u16::try_from(num.as_u64().unwrap()).unwrap(),
            Value::String(num) => num.parse::<u16>().unwrap(),
            _ => panic!("elo has invalid type"),
        });
        let stats = self.stats.into_iter();
        let stats = stats
            .filter_map(|(k, v)| match v {
                Value::String(str) => Some((k, str)),
                _ => None,
            })
            .collect();

        Match {
            match_id: self.match_id,
            date,
            elo,
            premade: self.premade,
            team_id: self.team_id,
            game_mode: self.game_mode,
            best_of: self.best_of.unwrap_or_default(),
            played: self.played.unwrap_or_default(),
            status: self.status,
            game: self.game,
            stats,
        }
    }
}

impl Into<Matches> for Response {
    fn into(self) -> Matches {
        Matches(self.0.into_iter().map(|m| m.into()).collect())
    }
}

impl Client {
    /// - `https://api.faceit.com/stats/v1/stats/time/users/{USER_ID}/games/csgo`
    pub async fn matches(
        &self,
        user_id: &str,
        size: usize,
        page: usize,
    ) -> reqwest::Result<Matches> {
        const PREFIX: &str = "https://api.faceit.com/stats/v1/stats/time/users";
        const SUFFIX: &str = "games/csgo";

        let size = size.to_string();
        let page = page.to_string();
        let query = [("size", size.as_str()), ("page", &page.as_str())];
        let url = format!("{}/{}/{}", PREFIX, user_id, SUFFIX);

        Ok(self.get_json::<Response>(&url, &query).await?.into())
    }
}

