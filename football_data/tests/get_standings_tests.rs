use football_data::{client::Client, dtos::StandingCollection};

#[test]
fn get_standings_with_invalid_auth_token() {
    let client = Client::new("SOME-INVALID-AUTH-TOKEN").unwrap();

    let result = client.get_standings(1).unwrap_err().message;

    assert_eq!(result, "Your API token is invalid.");
}

#[test]
fn get_standings() {
    let client = Client::new("SOME-AUTH-TOKEN").unwrap();

    let result = client.get_standings(1);
}
