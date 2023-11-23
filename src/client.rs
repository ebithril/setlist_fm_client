use core::fmt;
use derivative::Derivative;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::data::*;
use crate::error::*;

pub enum Sort {
    SortName,
    Relevance,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SortName => write!(f, "sortName"),
            Self::Relevance => write!(f, "relevance"),
        }
    }
}

pub struct SetlistFMClient {
    client: reqwest::Client,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct ArtistSetlistArgs {
    pub mbid: String,
    #[derivative(Default(value = "1"))]
    pub p: u32,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct SearchArtistArgs {
    pub artist_mbid: Option<String>,
    pub artist_name: Option<String>,
    pub artist_tmid: Option<String>,
    #[derivative(Default(value = "1"))]
    pub p: u32,
    #[derivative(Default(value = "Sort::SortName"))]
    pub sort: Sort,
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
                .expect("Failed to create reqwest client"),
        }
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: HashMap<String, String>,
    ) -> Result<T> {
        let url = Url::parse_with_params(
            &format!("https://api.setlist.fm/rest/1.0/{}", endpoint),
            params.iter(),
        )?;
        let result = self.client.get(url).send().await?;

        match result.error_for_status() {
            Ok(res) => Ok(res.json::<T>().await?),
            Err(err) => Err(SetlistError::from(err)),
        }
    }

    pub async fn artist(&self, mbid: &str) -> Result<Artist> {
        self.send_request(&format!("artist/{}", mbid), HashMap::new())
            .await
    }

    pub async fn artist_setlists(&self, args: &ArtistSetlistArgs) -> Result<SetlistResult> {
        let params = HashMap::from([("p".to_string(), args.p.to_string())]);

        self.send_request(&format!("artist/{}/setlists", args.mbid), params)
            .await
    }

    /// Unclear if this works, I don't know how to get a correct geo_id
    pub async fn city(&self, geo_id: &str) -> Result<City> {
        self.send_request(&format!("city/{}", geo_id), HashMap::new())
            .await
    }

    pub async fn search_artist(&self, args: &SearchArtistArgs) -> Result<ArtistSearchResult> {
        let mut params = HashMap::from([
            ("p".to_string(), args.p.to_string()),
            ("sort".to_string(), args.sort.to_string()),
        ]);

        if args.artist_name.is_some() {
            params.insert(
                "artistName".to_string(),
                args.artist_name.as_ref().unwrap().to_string(),
            );
        }

        if args.artist_mbid.is_some() {
            params.insert(
                "artistMbid".to_string(),
                args.artist_mbid.as_ref().unwrap().to_string(),
            );
        }

        if args.artist_tmid.is_some() {
            params.insert(
                "artistTmid".to_string(),
                args.artist_tmid.as_ref().unwrap().to_string(),
            );
        }

        self.send_request("search/artists", params).await
    }

    pub async fn search_cities(&self, name: &str) -> Result<CitySearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("name".to_string(), name.to_string()),
        ]);

        self.send_request("search/cities", params).await
    }

    pub async fn search_countries(&self) -> Result<CountrySearchResult> {
        self.send_request("search/countries", HashMap::new()).await
    }

    pub async fn get_city(&self, geo_id: &str) -> Result<City> {
        self.send_request(&format!("city/{}", geo_id), HashMap::new())
            .await
    }
}
