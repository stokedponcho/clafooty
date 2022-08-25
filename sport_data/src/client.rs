use reqwest::{blocking, header, StatusCode, Url};
use serde::Deserialize;

use crate::dtos::Error;

pub struct Client {
    inner_client: blocking::Client,
    base_url: Url,
}

impl Client {
    pub fn new(auth_token: &str, base_url: Option<Url>) -> Result<Self, String> {
        if auth_token.trim().is_empty() {
            return Err(format!("Invalid authentication token: {}", auth_token));
        }

        let mut headers = header::HeaderMap::new();

        headers.insert(
            "X-Auth-Token",
            header::HeaderValue::from_str(auth_token).unwrap(),
        );

        let inner_client = blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let base_url = match base_url {
            Some(url) => url,
            None => Url::parse("https://app.sportdataapi.com/api/v1/soccer/").unwrap(),
        };

        Ok(Client {
            inner_client,
            base_url,
        })
    }

    pub fn get<T>(&self, path: &str) -> Result<T, Error>
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
            StatusCode::NOT_FOUND => Err(Error {
                error: format!("{} {}", uri, status),
                error_code: Some(status.as_u16()),
            }),
            _ => {
                let mut error = response.json::<Error>().expect("Error - Could not parse");
                if error.error_code == None {
                    error.error_code = Some(status.as_u16());
                }
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_token() {
        assert!(Client::new("   ", None).is_err());
    }

    #[test]
    fn test_valid_token() {
        assert!(Client::new("a-token", None).is_ok());
    }
}
