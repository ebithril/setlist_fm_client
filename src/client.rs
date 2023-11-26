use core::fmt;
use derivative::Derivative;
use http_query_params::HttpQueryParams;
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

#[derive(Derivative, HttpQueryParams)]
#[derivative(Default)]
#[case(camelCase)]
pub struct ArtistSetlistArgs {
    #[derivative(Default(value = "1"))]
    pub p: u32,
}

#[derive(Derivative, HttpQueryParams)]
#[derivative(Default)]
#[case(camelCase)]
pub struct SearchArtistsArgs {
    pub artist_mbid: Option<String>,
    pub artist_name: Option<String>,
    pub artist_tmid: Option<String>,
    #[derivative(Default(value = "1"))]
    pub p: u32,
    #[derivative(Default(value = "Sort::SortName"))]
    pub sort: Sort,
}

#[derive(Derivative, HttpQueryParams)]
#[derivative(Default)]
#[case(camelCase)]
pub struct SearchCitiesArgs {
    pub country: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub state_code: Option<String>,
    #[derivative(Default(value = "1"))]
    pub p: u32,
}

#[derive(Derivative, HttpQueryParams)]
#[derivative(Default)]
#[case(camelCase)]
pub struct SearchSetlistsArgs {
    pub artist_mbid: Option<String>,
    pub artist_name: Option<String>,
    pub artist_tmid: Option<String>,
    pub city_id: Option<String>,
    pub city_name: Option<String>,
    pub country_code: Option<String>,
    pub date: Option<String>,
    pub last_fm: Option<String>,
    pub last_updated: Option<String>,
    #[derivative(Default(value = "1"))]
    pub p: u32,
    pub state: Option<String>,
    pub state_code: Option<String>,
    pub tour_name: Option<String>,
    pub venue_id: Option<String>,
    pub venue_name: Option<String>,
    pub year: Option<String>,
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

    pub async fn artist_setlists(
        &self,
        mbid: &str,
        args: &ArtistSetlistArgs,
    ) -> Result<SetlistResult> {
        let params = args.as_map();

        self.send_request(&format!("artist/{}/setlists", mbid), params)
            .await
    }

    pub async fn city(&self, geo_id: &str) -> Result<City> {
        self.send_request(&format!("city/{}", geo_id), HashMap::new())
            .await
    }

    pub async fn search_artists(&self, args: &SearchArtistsArgs) -> Result<ArtistSearchResult> {
        let params = args.as_map();

        self.send_request("search/artists", params).await
    }

    pub async fn search_cities(&self, args: &SearchCitiesArgs) -> Result<CitySearchResult> {
        let params = args.as_map();

        self.send_request("search/cities", params).await
    }

    pub async fn search_countries(&self) -> Result<CountrySearchResult> {
        self.send_request("search/countries", HashMap::new()).await
    }

    pub async fn search_setlists(&self, args: &SearchSetlistsArgs) -> Result<SetlistResult> {
        let params = args.as_map();

        self.send_request("search/setlists", params).await
    }
}
