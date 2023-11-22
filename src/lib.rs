//! # setlist_fm_client
//!
//! The `setlist_fm_client` crate aims to implement all of the setlist.fm api in a convinient rust
//! package.
//!
//! # Search Artist
//!
//! You can use [`search_artist`][SetlistFMClient::search_artist] to search for artist by name
//! ```
//! let client = SetlistFMClient::new("<your_api_key>");
//! let artists = client.search_artist("<artist_name>").await?;
//! ```
//!
//! # Get Artists Setlists
//!
//! You can use [`get_setlists`][SetlistFMClient::get_setlists] to get setlists that the artist has
//! played
//! ```
//! let client = SetlistFMClient::new("<your_api_key>");
//! let setlists = client.get_setlist("<artist_mbid>").await?;
//! ```

mod client;
mod data;
mod error;

pub use self::client::*;
pub use self::data::*;
pub use self::error::{Result, SetlistError};
