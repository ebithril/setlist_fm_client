use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use std::str;
use http::StatusCode;
use std::fmt;

pub type Result<T> = std::result::Result<T, SetlistError>;

pub struct SetlistFMClient {
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct SetlistError {
    pub status: StatusCode,
    pub message: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub mbid: String,
    pub name: String,
    pub sortName: String,
    pub disambiguation: String,
    pub url: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Venue {
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Tour {
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Set {
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Setlist {
    pub id: String,
    pub eventDate: String,
    pub lastUpdated: String,
    pub artist: Artist,
    pub venue: Venue,
    pub tour: Tour,
    pub sets: HashMap<String, Set>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArtistSearchResult {
    pub artist: Vec<Artist>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SetlistResult {
    pub setlist: Vec<Setlist>,
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

