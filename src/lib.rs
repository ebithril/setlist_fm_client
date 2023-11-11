use std::env;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use std::str;
use http::StatusCode;
use std::fmt;
use std::error::Error;
use std::{thread, time};

pub type Result<T> = std::result::Result<T, SetlistError>;

pub struct SetlistFMClient {
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct SetlistError {
    status: StatusCode,
    message: String
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
        let url = format!("https://api.setlist.fm/rest/1.0/search/artists?artistName={}&p=1&sort=sortName", artist_name);

        let result = self.client.get(url).send().await.expect("Failed to search artist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }

        Ok(result.json::<ArtistSearchResult>().await.expect("failed to serialize json"))
    }

    pub async fn get_setlists(&self, mbid: &String) -> Result<SetlistResult> {
        let url = format!("https://api.setlist.fm/rest/1.0/artist/{}/setlists?p=1", mbid);

        let result = self.client.get(url).send().await.expect("Failed to get setlist");

        if !result.status().is_success() {
            return Err(SetlistError::new(result.status(), result.text().await.expect("couldn't get text")));
        }


        Ok(result.json::<SetlistResult>().await.expect("failed to serialize json"))
    }
}

impl SetlistError {
    fn new(status: StatusCode, message: String) -> Self {
        SetlistError {
            status,
            message
        }
    }
}

impl fmt::Display for SetlistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusCode: {} Error: {}", self.status.as_str(), self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_artist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
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
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(api_key);

        let result = client.search_artist("Halestorm".to_string()).await.expect("Failed to find artist");

        thread::sleep(time::Duration::new(1, 0)); // Basic API key is limited to 2 requests/second

        for artist in &result.artist {
            if artist.name != "Halestorm" {
                continue;
            }

            let setlists = client.get_setlists(&artist.mbid).await.expect("Failed to get setlist");
            assert_eq!(setlists.setlist.len(), 1);
            break;
        }
    }
}
