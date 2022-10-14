use crate::client::Client;
use crate::shared::unix_time_ms;
use crate::MapStats;

use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Player {
    nickname: String,
    #[serde(rename = "playerId")]
    player_id: String,
    #[serde(flatten)]
    stats: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Team_ {
    premade: bool,
    #[serde(rename = "teamId")]
    team_id: String,
    players: Vec<Player>,
    #[serde(flatten)]
    stats: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct MatchStats_ {
    #[serde(rename = "bestOf")]
    best_of: String,
    date: u64,
    created_at: u64,
    updated_at: u64,
    #[serde(rename = "competitionId")]
    competition_id: String,
    game: String,
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "matchId")]
    match_id: String,
    #[serde(rename = "matchRound")]
    match_round: String,
    played: String,
    teams: Vec<Team_>,
    #[serde(flatten)]
    stats: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Response(Vec<MatchStats_>);

#[derive(Serialize, Debug)]
struct Team {
    team_id: String,
    players: Vec<Player>,
    stats: HashMap<String, String>,
}

impl Into<Team> for Team_ {
    fn into(self) -> Team {
        Team {
            team_id: self.team_id,
            players: self.players,
            stats: self.stats,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MatchStats {
    date: DateTime<Local>,
    game: String,
    game_mode: String,
    match_id: String,
    played: String,
    teams: Vec<Team>,
    stats: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct RoomStats(pub Vec<MatchStats>);

impl MapStats for RoomStats {
    fn map_stats(&mut self, mapping: &crate::Mapping) {
        self.0.iter_mut().for_each(|stats| {
            stats.stats.map_stats(mapping);
            stats.teams.iter_mut().for_each(|team| {
                team.stats.map_stats(mapping);
                team.players.iter_mut().for_each(|player| {
                    player.stats.map_stats(mapping);
                })
            })
        })
    }
}

impl Into<RoomStats> for Response {
    fn into(self) -> RoomStats {
        RoomStats(self.0.into_iter().map(|m| m.into()).collect())
    }
}

impl Into<MatchStats> for MatchStats_ {
    fn into(self) -> MatchStats {
        let date = unix_time_ms(self.date);
        let teams = self.teams.into_iter().map(|team| team.into()).collect();

        MatchStats {
            date,
            game: self.game,
            game_mode: self.game_mode,
            match_id: self.match_id,
            played: self.played,
            teams,
            stats: self.stats,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/stats/v1/stats/matches/{MATCH_ID}`
    pub async fn room_stats(&self, room_id: &str) -> reqwest::Result<RoomStats> {
        const PREFIX: &str = "https://api.faceit.com/stats/v1/stats/matches";

        let url = format!("{}/{}", PREFIX, room_id);
        Ok(self.get_json::<Response>(&url, &[]).await?.into())
    }
}

