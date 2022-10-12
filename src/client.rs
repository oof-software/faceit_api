use serde::de::DeserializeOwned;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    pub(crate) async fn get_json<T>(&self, url: &str, query: &[(&str, &str)]) -> reqwest::Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .get(url)
            .query(query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
