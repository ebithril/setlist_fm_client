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
    pub fn new(api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
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
            .await?;

        match result.error_for_status() {
            Ok(res) => Ok(res.json::<T>().await?),
            Err(err) => Err(SetlistError::from(err))
        }
    }

    pub async fn search_artist(&self, artist_name: &str) -> Result<ArtistSearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("artistName".to_string(), artist_name.to_string())
        ]);

        self.send_request("search/artists", params).await
    }

    pub async fn search_cities(&self, name: &str) -> Result<CitySearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("name".to_string(), name.to_string())
        ]);

        self.send_request("search/cities", params).await
    }

    pub async fn get_setlists(&self, mbid: &str) -> Result<SetlistResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string())
        ]);

        self.send_request(&format!("artist/{}/setlists", mbid), params).await
    }
}
