use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use url_search_params::build_url_search_params;

use crate::error::*;
use crate::data::*;

pub struct SetlistFMClient {
    client: reqwest::Client,
}

fn build_url(endpoint: &str, params: HashMap<String, String>) -> String {
    let query_parameters = build_url_search_params(params); 
    format!("https://api.setlist.fm/rest/1.0/{}?{}", endpoint, query_parameters)
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

    pub async fn search_artist(&self, artist_name: String) -> Result<ArtistSearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("artistName".to_string(), artist_name.clone())
        ]);

        let result = self.client.get(build_url("search/artists", params))
            .send()
            .await
            .expect("Failed to search artist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }

        Ok(result.json::<ArtistSearchResult>().await.expect("failed to serialize json"))
    }

    pub async fn get_setlists(&self, mbid: &String) -> Result<SetlistResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string())
        ]);

        let result = self.client.get(build_url(&format!("artist/{}/setlists", mbid), params))
            .send()
            .await
            .expect("Failed to get setlist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }

        Ok(result.json::<SetlistResult>().await.expect("failed to serialize json"))
    }

    pub async fn get_user(&self, user_id: String) -> Result<UserResult> {
        let url = format!("https://api.setlist.fm/rest/1.0/user/{}", user_id);
        let result = self.client.get(url).send().await.expect("Failed to get setlist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }

        Ok(result.json::<UserResult>().await.expect("failed to serialize json"))
    }
}
