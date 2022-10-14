use crate::client::Client;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct PickBan {
    guid: String,
    status: String,
    random: bool,
    round: u8,
    selected_by: String,
}

#[derive(Deserialize)]
struct Sequence {
    entity_type: String,
    vote_type: String,
    entities: Vec<PickBan>,
}

#[derive(Deserialize)]
struct Payload {
    match_id: String,
    tickets: Vec<Sequence>,
}

#[derive(Deserialize)]
struct Response {
    payload: Payload,
}

#[derive(Serialize, Debug)]
pub struct Democracy {
    match_id: String,
    map_veto: Vec<PickBan>,
}

impl Into<Democracy> for Response {
    fn into(self) -> Democracy {
        let pl = self.payload;
        let map_veto = pl
            .tickets
            .into_iter()
            .find(|seq| seq.entity_type == "map")
            .map(|seq| seq.entities)
            .unwrap_or_default();

        Democracy {
            match_id: pl.match_id,
            map_veto,
        }
    }
}

impl Client {
    /// - `https://api.faceit.com/democracy/v1/match/{MATCH_ID}/history`
    pub async fn veto(&self, room_id: &str) -> reqwest::Result<Democracy> {
        const PREFIX: &str = "https://api.faceit.com/democracy/v1/match";
        const SUFFIX: &str = "history";

        let url = format!("{}/{}/{}", PREFIX, room_id, SUFFIX);

        Ok(self.get_json::<Response>(&url, &[]).await?.into())
    }
}
