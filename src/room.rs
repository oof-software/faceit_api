use crate::{client::Client, shared::parse_rfc3339};

use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Duration, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EntityCustom {
    #[serde(rename = "effectiveRanking")]
    effective_ranking: Option<f32>,
    parties: HashMap<String, Vec<String>>,
    #[serde(rename = "partyQueueDurations")]
    party_queue_durations: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
struct Player {
    id: String,
    nickname: String,
    avatar: Option<String>,
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "gameName")]
    game_name: Option<String>,
    elo: u16,
    #[serde(rename = "gameSkillLevel")]
    game_skill_level: u8,
    #[serde(rename = "acReq")]
    anti_cheat_required: bool,
    #[serde(rename = "partyId")]
    party_id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SkillLevelRange {
    min: u8,
    max: u8,
}

#[derive(Deserialize, Debug)]
struct SkillLevel {
    average: u8,
    range: SkillLevelRange,
}

#[derive(Deserialize, Debug)]
struct Stats {
    rating: u16,
    #[serde(rename = "winProbability")]
    win_probability: f32,
    #[serde(rename = "skillLevel")]
    skill_level: SkillLevel,
}

#[derive(Deserialize, Debug)]
struct Team {
    id: String,
    name: String,
    leader: String,
    roster: Vec<Player>,
    stats: Option<Stats>,
    substituted: bool,
}

#[derive(Deserialize, Debug)]
struct Teams {
    #[serde(rename = "faction1")]
    faction_1: Team,
    #[serde(rename = "faction2")]
    faction_2: Team,
}

#[derive(Deserialize, Debug)]
struct Server {
    country: String,
    ip: String,
    port: String,
}

#[derive(Deserialize, Debug)]
struct ClientCustom {
    match_id: String,
    #[serde(rename = "team1_score")]
    team_1_score: u8,
    #[serde(rename = "team2_score")]
    team_2_score: u8,
    map: String,
    server: Server,
}

#[derive(Deserialize, Debug)]
struct SummaryResults {
    leavers: Vec<String>,
    afk: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct VotingMap {
    pick: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Voting {
    map: VotingMap,
}

#[derive(Deserialize, Debug)]
struct Payload {
    id: String,
    game: String,
    region: String,
    #[serde(rename = "organizerId")]
    organizer_id: String,
    #[serde(rename = "startedAt")]
    started_at: String,
    #[serde(rename = "configuredAt")]
    configured_at: String,
    #[serde(rename = "finishedAt")]
    finished_at: String,
    #[serde(rename = "timeToConnect")]
    time_to_connect: usize,
    version: usize,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "lastModified")]
    last_modified: String,
    #[serde(rename = "anticheatRequired")]
    anti_cheat_required: bool,
    #[serde(rename = "anticheatMode")]
    anti_cheat_mode: String,
    state: String,
    status: String,
    teams: Teams,
    #[serde(rename = "clientCustom")]
    client_custom: Option<ClientCustom>,
    #[serde(rename = "summaryResults")]
    summary_results: SummaryResults,
    voting: Voting,
    #[serde(rename = "entityCustom")]
    entity_custom: EntityCustom,
}

#[derive(Deserialize, Debug)]
struct Response {
    time: u64,
    version: String,
    payload: Payload,
}

pub struct Room {
    id: String,
    maps: Vec<String>,
    started_at: DateTime<Local>,
    configured_at: DateTime<Local>,
    finished_at: DateTime<Local>,
    party_queue_durations: HashMap<String, Duration>,
}

impl Room {
    pub fn duration(&self) -> Duration {
        self.finished_at.signed_duration_since(self.started_at)
    }
    pub fn longest_queue_duration(&self) -> Duration {
        self.party_queue_durations
            .values()
            .max()
            .map_or(Duration::zero(), |d| d.to_owned())
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dur = self.duration();
        let dur = format!(
            "[{:02}:{:02}:{:02}]",
            dur.num_hours(),
            dur.num_minutes() % 60,
            dur.num_seconds() % 60
        );
        let queue_dur = self.longest_queue_duration();
        let queue_dur = format!(
            "[{:02}:{:02}:{:02}]",
            queue_dur.num_hours(),
            queue_dur.num_minutes() % 60,
            queue_dur.num_seconds() % 60
        );
        f.debug_struct("Room")
            .field("id", &self.id)
            .field("duration", &dur)
            .field("maps", &self.maps)
            .field("max_queue", &queue_dur)
            .finish()
    }
}

impl Into<Room> for Response {
    fn into(self) -> Room {
        let pl = self.payload;
        let queue_durations = pl.entity_custom.party_queue_durations.into_iter();
        let queue_durations = queue_durations
            .map(|(k, v)| (k, Duration::milliseconds((v * 1e3) as i64)))
            .collect();

        Room {
            id: pl.id,
            maps: pl.voting.map.pick,
            started_at: parse_rfc3339(&pl.started_at),
            configured_at: parse_rfc3339(&pl.configured_at),
            finished_at: parse_rfc3339(&pl.finished_at),
            party_queue_durations: queue_durations,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/match/v2/match/{MATCH_ID}`
    pub async fn room(&self, room_id: &str) -> reqwest::Result<Room> {
        const PREFIX: &str = "https://api.faceit.com/match/v2/match";

        let url = format!("{}/{}", PREFIX, room_id);

        Ok(self.get_json::<Response>(&url, &[]).await?.into())
    }
}

