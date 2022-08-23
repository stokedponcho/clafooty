use reqwest::{blocking, header, StatusCode, Url};
use serde::Deserialize;

use crate::dtos::ErrorMessage;

pub struct Client {
    inner_client: blocking::Client,
    base_url: Url,
}

impl Client {
    pub fn new(auth_token: &str) -> Result<Self, String> {
        if auth_token.trim().is_empty() {
            return Err(format!("Invalid authentication token: {}", auth_token));
        }

        let base_url = Url::parse("https://api.football-data.org/v2/").unwrap();
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "X-Auth-Token",
            header::HeaderValue::from_str(auth_token).unwrap(),
        );

        let inner_client = blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Client {
            inner_client,
            base_url,
        })
    }

    pub fn get<T>(&self, path: &str) -> Result<T, ErrorMessage>
    where
        T: for<'de> Deserialize<'de>,
    {
        let uri = self.base_url.join(path).unwrap();
        let uri = uri.as_str();
        let response = self
            .inner_client
            .get(uri)
            .send()
            .expect("Could not complete request");
        let status = response.status();

        match status {
            StatusCode::OK => Ok(response.json::<T>().expect("OK - Could not parse")),
            StatusCode::NOT_FOUND => Err(ErrorMessage {
                message: format!("{} {}", uri, status),
                error_code: status.as_u16(),
            }),
            _ => Err(response
                .json::<ErrorMessage>()
                .expect("Error - Could not parse")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_token() {
        assert!(Client::new("   ").is_err());
    }

    #[test]
    fn test_valid_token() {
        assert!(Client::new("a-token").is_err() == false);
    }
}
