use crate::client::Client;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "n")]
    nickname: String,
}

#[derive(Debug, Serialize)]
pub struct Nickname {
    pub id: String,
    pub nickname: String,
}

impl Client {
    /// - `https://chat-server.faceit.com/vcards/{USER_ID}`
    pub async fn nickname(&self, user_id: &str) -> reqwest::Result<Nickname> {
        const PREFIX: &str = "https://chat-server.faceit.com/vcards";

        let url = format!("{}/{}", PREFIX, user_id);

        let resp = self.get_json::<Response>(&url, &[]).await?;
        Ok(Nickname {
            id: user_id.to_string(),
            nickname: resp.nickname,
        })
    }
}
