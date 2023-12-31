#[cfg(test)]
mod tests {
    use http::StatusCode;
    use setlist_fm_client::*;
    use std::env;
    use tokio::time;

    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(750);

    // Some constants to search for in the tests
    const ARTIST_NAME: &str = "Halestorm";
    const VENUE_NAME: &str = "Münchenbryggeriet";
    const CITY_NAME: &str = "Stockholm";
    const COUNTRY_NAME: &str = "Sweden";
    const GEO_ID: &str = "8126189";

    #[tokio::test]
    async fn api_key_error() {
        let client = SetlistFMClient::new("bad api key");

        time::sleep(SLEEP_DURATION).await;
        let result = client
            .search_artists(&SearchArtistsArgs {
                artist_name: Some(ARTIST_NAME.to_string()),
                ..Default::default()
            })
            .await;

        match result {
            Ok(_) => {
                panic!("This should not return a valid result");
            }
            Err(err) => match err {
                SetlistError::Reqwest(ref err) => {
                    assert_eq!(err.status(), Some(StatusCode::FORBIDDEN))
                }
                _ => panic!("Unexpected error type"),
            },
        }
    }

    #[tokio::test]
    async fn artist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await;
        let result = client
            .search_artists(&SearchArtistsArgs {
                artist_name: Some(ARTIST_NAME.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();

        for artist in &result.artist {
            if artist.name != ARTIST_NAME {
                continue;
            }

            time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
            let artist_res = client
                .artist(&artist.mbid)
                .await
                .expect("Failed to get artist");
            assert_eq!(artist_res.name, artist.name);
            break;
        }
    }

    #[tokio::test]
    async fn artist_setlist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client
            .search_artists(&SearchArtistsArgs {
                artist_name: Some(ARTIST_NAME.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();

        for artist in &result.artist {
            if artist.name != ARTIST_NAME {
                continue;
            }

            time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
            let setlists = client
                .artist_setlists(&artist.mbid, &ArtistSetlistArgs::default())
                .await
                .expect("Failed to get setlist");
            assert_eq!(setlists.setlist.len(), 20);
            break;
        }
    }

    #[tokio::test]
    async fn city() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client.city(GEO_ID).await.expect("Failed to get city");

        assert_eq!(GEO_ID, result.id);
    }

    #[tokio::test]
    async fn search_artist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client
            .search_artists(&SearchArtistsArgs {
                artist_name: Some(ARTIST_NAME.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();

        let mut found = false;
        for artist in &result.artist {
            if artist.name != ARTIST_NAME {
                continue;
            }

            found = true;
            break;
        }

        assert!(found);
    }

    #[tokio::test]
    async fn search_cities() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client
            .search_cities(&SearchCitiesArgs {
                name: Some(CITY_NAME.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();

        let mut found = false;
        for artist in &result.cities {
            if artist.name != CITY_NAME {
                continue;
            }

            found = true;
            break;
        }

        assert!(found);
    }

    #[tokio::test]
    async fn search_countries() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client.search_countries().await.unwrap();

        let mut found = false;
        for country in result.country {
            if country.name != COUNTRY_NAME {
                continue;
            }

            found = true;
        }

        assert!(found);
    }

    #[tokio::test]
    async fn search_setlists() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client
            .search_setlists(&SearchSetlistsArgs {
                artist_name: Some(ARTIST_NAME.to_string()),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(result.setlist.len(), 20);
    }

    #[tokio::test]
    async fn search_venues() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(&api_key);

        time::sleep(SLEEP_DURATION).await; // Basic API key is limited to 2 requests/second
        let result = client
            .search_venues(&SearchVenuesArgs{name: Some(VENUE_NAME.to_string()), ..Default::default()})
            .await
            .unwrap();

        let mut found = false;
        for venue in result.venue {
            if venue.name != VENUE_NAME {
                continue;
            }

            found = true;
        }

        assert!(found);
    }
}
