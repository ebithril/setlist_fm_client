
#[cfg(test)]
mod tests {
    use setlist_fm_client::*;
    use std::{thread, time};
    use std::env;
    use http::StatusCode;

    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(2000);

    #[tokio::test]
    async fn search_artist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(api_key);

        thread::sleep(SLEEP_DURATION); // Basic API key is limited to 2 requests/second
        let result = client.search_artist("Halestorm".to_string()).await.unwrap();

        let mut found = false;
        for artist in &result.artist {
            if artist.name != "Halestorm" {
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
        let client = SetlistFMClient::new(api_key);

        thread::sleep(SLEEP_DURATION); // Basic API key is limited to 2 requests/second
        let result = client.search_cities("Stockholm".to_string()).await.unwrap();

        let mut found = false;
        for artist in &result.cities {
            if artist.name != "Stockholm" {
                continue;
            }

            found = true;
            break;
        }

        assert!(found);
    }

    #[tokio::test]
    async fn get_setlist() {
        let api_key = env::var("API_KEY").expect("Could not find environment var");
        let client = SetlistFMClient::new(api_key);

        thread::sleep(SLEEP_DURATION); // Basic API key is limited to 2 requests/second
        let result = client.search_artist("Halestorm".to_string()).await.expect("Failed to find artist");

        for artist in &result.artist {
            if artist.name != "Halestorm" {
                continue;
            }

            thread::sleep(SLEEP_DURATION); // Basic API key is limited to 2 requests/second
            let setlists = client.get_setlists(&artist.mbid).await.expect("Failed to get setlist");
            assert_eq!(setlists.setlist.len(), 20);
            break;
        }
    }

    #[tokio::test]
    async fn api_key_error() {
        let client = SetlistFMClient::new("bad api key".to_string());

        thread::sleep(SLEEP_DURATION); // Basic API key is limited to 2 requests/second
        let result = client.search_artist("anything".to_string()).await;
        match result {
            Ok(_) => {
                panic!("This should not return a valid result");
            },
            Err(err) => {
                match err {
                    SetlistError::Reqwest(ref err) => assert_eq!(err.status(), Some(StatusCode::FORBIDDEN))
                }
            }
        }
    }
}
