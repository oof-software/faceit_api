use crate::{client::Client, shared::parse_rfc3339};

use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SteamPlatform {
    id: String,
    id64: String,
    nickname: String,
}

#[derive(Deserialize)]
struct Platform {
    steam: SteamPlatform,
}

#[derive(Deserialize)]
struct GameInfo {
    game_id: String,
    faceit_elo: u16,
    region: String,
    skill_level: u8,
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct Payload {
    id: String,
    activated_at: String,
    active_team_id: String,
    country: String,
    avatar: String,
    created_at: String,
    flag: String,
    friends: Vec<String>,
    games: HashMap<String, GameInfo>,
    gender: String,
    matching_sound: String,
    memberships: Vec<String>,
    nickname: String,
    phone_verified: bool,
    platforms: Platform,
}

#[derive(Deserialize)]
struct Response {
    payload: Payload,
}

#[derive(Debug, Serialize)]
pub struct PlayerInfo {
    id: String,
    country: String,
    avatar: String,
    created_at: DateTime<Local>,
    friends: Vec<String>,
    gender: String,
    matching_sound: String,
    memberships: Vec<String>,
    nickname: String,
    phone_verified: bool,
}

impl PlayerInfo {
    pub fn account_age(&self, now: &DateTime<Local>) -> Duration {
        now.signed_duration_since(self.created_at)
    }
}

impl Into<PlayerInfo> for Response {
    fn into(self) -> PlayerInfo {
        let pl = self.payload;
        let created_at = parse_rfc3339(&pl.created_at);

        PlayerInfo {
            id: pl.id,
            country: pl.country,
            avatar: pl.avatar,
            created_at,
            friends: pl.friends,
            gender: pl.gender,
            matching_sound: pl.matching_sound,
            memberships: pl.memberships,
            nickname: pl.nickname,
            phone_verified: pl.phone_verified,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/users/v1/nicknames/{NICKNAME}`
    pub async fn info(&self, nickname: &str) -> reqwest::Result<PlayerInfo> {
        const PREFIX: &str = "https://api.faceit.com/users/v1/nicknames";

        let url = format!("{}/{}", PREFIX, nickname);

        Ok(self.get_json::<Response>(&url, &[]).await?.into())
    }
}

