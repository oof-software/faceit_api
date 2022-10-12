use crate::{client::Client, shared::unix_time_ms};

use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize)]
struct Game {
    name: String,
    skill_level: u8,
}

#[derive(Deserialize)]
struct Player {
    id: String,
    guid: String,
    nickname: String,
    status: String,
    country: String,
    verified: bool,
    games: Vec<Game>,
}

#[derive(Deserialize)]
struct Players {
    total_count: usize,
    results: Vec<Player>,
}

#[derive(Deserialize)]
struct Payload {
    offset: usize,
    limit: usize,
    players: Players,
}

#[derive(Deserialize)]
struct Response {
    time: u64,
    payload: Payload,
}

#[derive(Debug)]
pub struct Search {
    pub time: DateTime<Local>,
    pub offset: usize,
    pub limit: usize,
    pub total_players: usize,
    pub players: Vec<String>,
}

impl Into<Search> for Response {
    fn into(self) -> Search {
        let time = unix_time_ms(self.time);
        let players = self.payload.players.results.into_iter();
        let players = players.map(|p| p.guid).collect();

        Search {
            time,
            offset: self.payload.offset,
            limit: self.payload.limit,
            total_players: self.payload.players.total_count,
            players,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/search/v1`
    pub async fn search(
        &self,
        query: &str,
        offset: usize,
        limit: usize,
    ) -> reqwest::Result<Search> {
        const API: &str = "https://api.faceit.com/search/v1/";

        let offset = offset.to_string();
        let limit = limit.to_string();
        let query = [("query", query), ("offset", &offset), ("limit", &limit)];

        Ok(self.get_json::<Response>(API, &query).await?.into())
    }
}

