use crate::client::Client;

use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "n")]
    nickname: String,
}

impl Client {
    /// - `https://chat-server.faceit.com/vcards/{USER_ID}`
    pub async fn nickname(&self, user_id: &str) -> reqwest::Result<String> {
        const PREFIX: &str = "https://chat-server.faceit.com/vcards";

        let url = format!("{}/{}", PREFIX, user_id);

        Ok(self.get_json::<Response>(&url, &[]).await?.nickname)
    }
}
