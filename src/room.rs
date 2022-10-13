use crate::client::Client;
use crate::shared::parse_rfc3339;

use std::time::Duration;
use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct EntityCustom {
    #[serde(rename = "effectiveRanking")]
    effective_ranking: Option<f32>,
    parties: HashMap<String, Vec<String>>,
    #[serde(rename = "partyQueueDurations")]
    party_queue_durations: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
struct Player_ {
    id: String,
    nickname: String,
    avatar: Option<String>,
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "gameName")]
    game_name: Option<String>,
    memberships: Vec<String>,
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
struct Stats_ {
    rating: u16,
    #[serde(rename = "winProbability")]
    win_probability: f32,
    #[serde(rename = "skillLevel")]
    skill_level: SkillLevel,
}

#[derive(Deserialize, Debug)]
struct Team_ {
    id: String,
    name: String,
    leader: String,
    roster: Vec<Player_>,
    stats: Option<Stats_>,
    substituted: bool,
}

#[derive(Deserialize, Debug)]
struct Teams_ {
    #[serde(rename = "faction1")]
    faction_1: Team_,
    #[serde(rename = "faction2")]
    faction_2: Team_,
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
    teams: Teams_,
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

#[derive(Serialize)]
pub struct Stats {
    win_probability: f32,
    rating: u16,
}

#[derive(Serialize)]
pub struct Player {
    id: String,
    nickname: String,
    game_id: String,
    elo: u16,
    skill_level: u8,
    memberships: Vec<String>,
}

#[derive(Serialize)]
pub struct Team {
    id: String,
    name: String,
    leader: String,
    roster: Vec<Player>,
}

#[derive(Serialize)]
pub struct Teams {
    faction_1: Team,
    faction_2: Team,
}

#[derive(Serialize)]
pub struct Room {
    id: String,
    maps: Vec<String>,
    started_at: DateTime<Local>,
    configured_at: DateTime<Local>,
    finished_at: DateTime<Local>,
    match_duration: f32,
    party_queue_durations: HashMap<String, f32>,
    parties: HashMap<String, Vec<String>>,
    teams: Teams,
}

impl Room {
    pub fn duration(&self) -> Duration {
        self.finished_at
            .signed_duration_since(self.started_at)
            .to_std()
            .unwrap()
    }
    pub fn longest_queue_duration(&self) -> f32 {
        self.party_queue_durations
            .values()
            .map(|&f| f)
            .reduce(f32::max)
            .unwrap_or_default()
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dur = self.duration().as_secs_f32();
        let queue_dur = self.longest_queue_duration();
        f.debug_struct("Room")
            .field("id", &self.id)
            .field("duration", &dur)
            .field("maps", &self.maps)
            .field("max_queue", &queue_dur)
            .finish()
    }
}

impl Into<Player> for Player_ {
    fn into(self) -> Player {
        Player {
            id: self.id,
            nickname: self.nickname,
            game_id: self.game_id,
            elo: self.elo,
            skill_level: self.game_skill_level,
            memberships: self.memberships,
        }
    }
}

impl Into<Team> for Team_ {
    fn into(self) -> Team {
        Team {
            id: self.id,
            name: self.name,
            leader: self.leader,
            roster: self.roster.into_iter().map(|p| p.into()).collect(),
        }
    }
}

impl Into<Teams> for Teams_ {
    fn into(self) -> Teams {
        Teams {
            faction_1: self.faction_1.into(),
            faction_2: self.faction_2.into(),
        }
    }
}

impl Into<Room> for Response {
    fn into(self) -> Room {
        let pl = self.payload;
        let started_at = parse_rfc3339(&pl.started_at);
        let finished_at = parse_rfc3339(&pl.finished_at);
        let configured_at = parse_rfc3339(&pl.configured_at);
        let match_duration = finished_at.signed_duration_since(started_at);
        let match_duration = match_duration.to_std().unwrap();

        Room {
            id: pl.id,
            maps: pl.voting.map.pick,
            started_at,
            configured_at,
            finished_at,
            match_duration: match_duration.as_secs_f32(),
            party_queue_durations: pl.entity_custom.party_queue_durations,
            parties: pl.entity_custom.parties,
            teams: pl.teams.into(),
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

