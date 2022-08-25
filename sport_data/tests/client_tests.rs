use reqwest::Url;

use sport_data::client::Client;

#[test]
fn get_with_invalid_auth_token() {
    let client = Client::new("SOME-INVALID-AUTH-TOKEN", base_url()).unwrap();

    let result = client.get::<u8>("/error").unwrap_err();

    assert_eq!(result.error_code, Some(400));
    assert_eq!(result.error, "No apikey provided.");
}

fn base_url() -> Option<Url> {
    Some(Url::parse("http://localhost:8589/").unwrap())
}
