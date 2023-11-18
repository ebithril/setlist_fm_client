use serde::Deserialize;
use std::str;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub mbid: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Coords {
    pub lat: f64,
    pub long: f64,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct City {
    pub id: String,
    pub name: String,
    pub state: String,
    pub state_code: String,
    pub coords: Coords,
    pub country: Country,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub city: City,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Tour {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Song {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Set {
    pub song: Vec<Song>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Sets {
    pub set: Vec<Set>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Setlist {
    pub id: String,
    pub event_date: String,
    pub last_updated: String,
    pub artist: Artist,
    pub venue: Venue,
    pub tour: Tour,
    pub sets: Sets,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ArtistSearchResult {
    pub artist: Vec<Artist>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SetlistResult {
    pub setlist: Vec<Setlist>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserResult {
    pub user_id: String,
    pub url: String,
}


