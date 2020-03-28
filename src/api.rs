//! Shared utility types and functions for the API
use super::http::refresh_tokens;
use serde::Deserialize;
use serde_repr::*;
use std::convert::From;
use std::env;

#[doc(hidden)]
pub fn v3(token: Option<&AccessToken>, url: String) -> String {
    match token {
        Some(token) => format!(
            "https://www.strava.com/api/v3/{}?access_token={}",
            url,
            token.get()
        ),
        None => format!("https://www.strava.com/api/v3/{}", url),
    }
}

/// Wrapper for endpoints that paginate
///
/// A Paginated<T> will be returned from any endpoint that supports paging. Provides methods for
/// fetching the next page and checking if more pages are available.
#[derive(Debug, Deserialize)]
pub struct Paginated<T> {
    page: usize,
    per_page: usize,
    url: String,
    data: Vec<T>,
}

impl<T> Paginated<T> {
    pub fn new(url: String, data: Vec<T>) -> Paginated<T> {
        Paginated {
            page: 1,
            per_page: 30,
            url: url,
            data: data,
        }
    }

    /// Get the next page of results
    ///
    /// **UNIMPLEMENTED**
    pub fn fetch_next_page(&self) -> Option<Paginated<T>> {
        unimplemented!();
    }

    /// Check if this is the last page
    pub fn last_page(&self) -> bool {
        self.per_page != self.data.len()
    }
}

/// The level of detail for the current resource
///
/// Detailed contains the most data and Meta the least.
#[derive(Debug, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum ResourceState {
    Unknown = 0,
    Meta = 1,
    Summary = 2,
    Detailed = 3,
}

impl Default for ResourceState {
    fn default() -> ResourceState {
        ResourceState::Unknown
    }
}

/// A strava.com api refresh token.
///
/// You'll need to register/login at https://www.strava.com/developers to get a token. This is
/// required to get an access token.
pub struct RefreshToken {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
}

impl RefreshToken {
    /// Create a RefreshToken from the supplied string
    pub fn new(refresh_token: String, client_id: String, client_secret: String) -> RefreshToken {
        RefreshToken {
            refresh_token: refresh_token,
            client_id: client_id,
            client_secret: client_secret,
        }
    }
}

/// A strava.com api access token. This is required for all requests.
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
}

impl AccessToken {
    /// Create an AccessToken from the supplied string
    pub fn new(token: String) -> AccessToken {
        AccessToken {
            access_token: token,
            refresh_token: "".to_string(),
        }
    }

    /// Create an AccessToken from the environment variable STRAVA_ACCESS_TOKEN
    pub fn new_from_env() -> Result<AccessToken, env::VarError> {
        match env::var("STRAVA_ACCESS_TOKEN") {
            Ok(token) => Ok(AccessToken::new(token)),
            Err(e) => Err(e),
        }
    }

    pub async fn get_current(refresh_token: &RefreshToken) -> super::error::Result<AccessToken> {
        Ok(refresh_tokens(refresh_token).await?)
    }

    /// Get the token underlying string
    ///
    /// This is used internally for building requests.
    // TODO implement Deref -> &str for AccessToken
    pub fn get(&self) -> &str {
        &self.access_token[..]
    }
}

impl<'a> From<&'a str> for AccessToken {
    fn from(s: &'a str) -> AccessToken {
        AccessToken {
            access_token: s.to_string(),
            refresh_token: "".to_string(),
        }
    }
}

#[cfg(test)]
mod resource_state_tests {
    use std::default::Default;

    use super::ResourceState;

    #[test]
    fn values() {
        assert_eq!(ResourceState::Meta as i32, 1);
        assert_eq!(ResourceState::Summary as i32, 2);
        assert_eq!(ResourceState::Detailed as i32, 3);
    }

    #[test]
    fn default() {
        let default_state: ResourceState = Default::default();
        assert_eq!(default_state, ResourceState::Unknown);
    }
}

#[cfg(test)]
mod paginated_tests {
    use super::Paginated;

    #[test]
    fn last_page() {
        let vec = (0..30).collect::<Vec<u8>>();
        let pager = Paginated::new("test".to_string(), vec);
        println!("{:?}", pager);
        assert_eq!(pager.last_page(), false);
    }
}
