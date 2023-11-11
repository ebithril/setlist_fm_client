use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use std::str;

pub struct SetlistFMClient {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    mbid: String,
    name: String,
    sortName: String,
    disambiguation: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Venue {
}

#[derive(Serialize, Deserialize)]
pub struct Tour {
}

#[derive(Serialize, Deserialize)]
pub struct Set {
}

#[derive(Serialize, Deserialize)]
pub struct Setlist {
    id: String,
    eventDate: String,
    lastUpdated: String,
    artist: Artist,
    venue: Venue,
    tour: Tour,
    sets: HashMap<String, Set>
}

#[derive(Serialize, Deserialize)]
pub struct ArtistSearchResult {
    artist: Vec<Artist>,
}

#[derive(Serialize, Deserialize)]
pub struct SetlistResult {
    setlist: Vec<Setlist>,
}

impl SetlistFMClient {
    pub fn new(api_key: String) -> Self {
        SetlistFMClient {
            api_key,
            client: reqwest::Client::new()
        }
    }

    pub async fn search_artist(&self, artist_name: String) -> Result<ArtistSearchResult> {
        let url = format!("https://api.setlist.fm/rest/1.0/search/artists?artistName={}&p=1&sort=sortName", artist_name);
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(self.api_key.as_str()).unwrap());
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        let result = self.client.get(url).headers(headers).send().await;
        let response = result.unwrap().text().await.unwrap();
        
        serde_json::from_str(&response)
    }

    pub async fn get_setlists(&self, mbid: &String) -> Result<SetlistResult> {
        let url = format!("https://api.setlist.fm/rest/1.0/artist/{}/setlists?p=1", mbid);
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(self.api_key.as_str()).unwrap());
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        let result = self.client.get(url).headers(headers).send().await;
        let response = result.unwrap().text().await.unwrap();

        serde_json::from_str(str::from_utf8(response.as_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_artist() {
        let api_key = env::var("API_KEY").unwrap();
        let client = SetlistFMClient::new(api_key);

        let result = client.search_artist("Halestorm".to_string()).await.unwrap();

        let mut found = false;
        for artist in &result.artist {
            if artist.name != "Halestorm" {
                continue;
            }

            found = true;
            break;
        }

        assert_eq!(found, true);
    }

    #[tokio::test]
    async fn get_setlist() {
        let api_key = env::var("API_KEY").unwrap();
        let client = SetlistFMClient::new(api_key);

        let result = client.search_artist("Halestorm".to_string()).await.unwrap();

        for artist in &result.artist {
            if artist.name != "Halestorm" {
                continue;
            }

            let setlists = client.get_setlists(&artist.mbid).await.unwrap();
            assert_eq!(setlists.setlist.len(), 1);
            break;
        }
    }
}
