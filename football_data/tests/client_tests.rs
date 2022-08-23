use reqwest::Url;

use football_data::client::Client;

#[test]
fn get_with_invalid_auth_token() {
    let client = Client::new("SOME-INVALID-AUTH-TOKEN", base_url()).unwrap();

    let result = client.get::<u8>("invalid-api-token").unwrap_err();

    assert_eq!(result.error_code, 400);
    assert_eq!(result.message, "Your API token is invalid.");
}

fn base_url() -> Option<Url> {
    Some(Url::parse("http://localhost:8489/").unwrap())
}
