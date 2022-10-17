use crate::{client::Client, shared::parse_rfc3339};

use std::fmt::Debug;

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SteamPlatform {
    id: String,
    id64: String,
    nickname: Option<String>,
}

#[derive(Deserialize)]
struct Platform {
    steam: SteamPlatform,
}

#[derive(Deserialize)]
struct GameInfo {
    game_id: String,
    game_name: String,
    faceit_elo: u16,
    region: String,
    skill_level: u8,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Games {
    csgo: GameInfo,
}

#[derive(Deserialize)]
struct Payload {
    id: String,
    activated_at: String,
    active_team_id: Option<String>,
    country: String,
    avatar: Option<String>,
    cover_image_url: Option<String>,
    created_at: String,
    flag: String,
    friends: Vec<String>,
    games: Games,
    gender: Option<String>,
    matching_sound: Option<String>,
    memberships: Vec<String>,
    nickname: String,
    phone_verified: bool,
    platforms: Option<Platform>,
}

#[derive(Deserialize)]
struct Response {
    payload: Payload,
}

#[derive(Debug, Serialize)]
pub struct PlayerInfo {
    id: String,
    country: String,
    region: String,
    avatar: Option<String>,
    cover_image: Option<String>,
    created_at: DateTime<Local>,
    friends: Vec<String>,
    gender: Option<String>,
    matching_sound: Option<String>,
    memberships: Vec<String>,
    nickname: String,
    phone_verified: bool,
    game_id: String,
    game_name: String,
    elo: u16,
    skill_level: u8,
}

impl PlayerInfo {
    pub fn account_age(&self, now: &DateTime<Local>) -> Duration {
        now.signed_duration_since(self.created_at)
    }
}

impl Into<PlayerInfo> for Response {
    fn into(self) -> PlayerInfo {
        let pl = self.payload;
        let cs = pl.games.csgo;
        let created_at = parse_rfc3339(&pl.created_at);

        PlayerInfo {
            id: pl.id,
            country: pl.country,
            region: cs.region,
            avatar: pl.avatar,
            cover_image: pl.cover_image_url,
            created_at,
            friends: pl.friends,
            gender: pl.gender,
            matching_sound: pl.matching_sound,
            memberships: pl.memberships,
            nickname: pl.nickname,
            phone_verified: pl.phone_verified,
            game_id: cs.game_id,
            game_name: cs.game_name,
            elo: cs.faceit_elo,
            skill_level: cs.skill_level,
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

