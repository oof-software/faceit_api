use crate::client::Client;

use std::collections::HashMap;
use std::mem;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Label {
    en: String,
}

#[derive(Deserialize)]
struct Mapping_ {
    label: Label,
}

#[derive(Deserialize)]
struct Response {
    id: String,
    mapping: HashMap<String, Mapping_>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(transparent)]
pub struct Mapping(pub HashMap<String, String>);

impl Into<Mapping> for Response {
    fn into(self) -> Mapping {
        let map = self.mapping.into_iter();
        let map = map.map(|(k, v)| (k, v.label.en)).collect();
        Mapping(map)
    }
}

pub trait MapStats {
    fn map_stats(&mut self, mapping: &Mapping);
}

impl MapStats for HashMap<String, String> {
    fn map_stats(&mut self, mapping: &Mapping) {
        *self = mem::take(self)
            .into_iter()
            .filter_map(|(k, v)| Some((mapping.0.get(&k)?.clone(), v)))
            .collect();
    }
}

impl Client {
    /// - `https://api.faceit.com/stats/v1/stats/configuration/csgo`
    pub async fn mapping(&self) -> reqwest::Result<Mapping> {
        const API: &str = "https://api.faceit.com/stats/v1/stats/configuration/csgo";
        let resp = self.get_json::<Response>(API, &[]).await;
        Ok(resp?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stats_parsing() -> reqwest::Result<()> {
        let client = Client::new();
        let resp = client.mapping().await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
