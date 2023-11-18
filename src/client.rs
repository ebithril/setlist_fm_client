use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use url_search_params::build_url_search_params;
use serde::de::DeserializeOwned;

use crate::error::*;
use crate::data::*;

pub struct SetlistFMClient {
    client: reqwest::Client,
}

impl SetlistFMClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(api_key.as_str()).unwrap());
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        SetlistFMClient {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to create reqwest client")
        }
    }

    async fn send_request<T: DeserializeOwned>(&self, endpoint: &str, params: HashMap<String, String>) -> Result<T> {
        let query_parameters = build_url_search_params(params); 
        let url = format!("https://api.setlist.fm/rest/1.0/{}?{}", endpoint, query_parameters);
        let result = self.client.get(url)
            .send()
            .await
            .expect("Failed to search artist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }

        Ok(result.json::<T>().await.expect("failed to serialize json"))
    }

    pub async fn search_artist(&self, artist_name: String) -> Result<ArtistSearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("artistName".to_string(), artist_name.clone())
        ]);

        self.send_request("search/artists", params).await
    }

    pub async fn get_setlists(&self, mbid: &String) -> Result<SetlistResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string())
        ]);

        self.send_request(&format!("artist/{}/setlists", mbid), params).await
    }
}
