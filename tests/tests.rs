
#[cfg(test)]
mod tests {
    use setlist_fm_client::*;
    use std::{thread, time};
    use std::env;

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
