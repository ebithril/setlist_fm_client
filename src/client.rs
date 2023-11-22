use derivative::Derivative;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::data::*;
use crate::error::*;

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

    pub async fn search_artist(&self, artist_name: &str) -> Result<ArtistSearchResult> {
        let params = HashMap::from([
            ("p".to_string(), "1".to_string()),
            ("sort".to_string(), "sortName".to_string()),
            ("artistName".to_string(), artist_name.to_string()),
        ]);

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
