use reqwest::Url;

use football_data::client::Client;

fn client() -> Client {
    Client::new(
        "SOME-AUTH-TOKEN",
        Some(Url::parse("http://localhost:8489/").unwrap()),
    )
    .unwrap()
}

#[test]
fn get_competitions() {
    let result = client().get_competition(1);

    assert!(result.is_ok());
}

#[test]
fn get_competition_fixtures() {
    let result = client().get_competition_matches(1, 1);

    assert!(result.is_ok());
}

#[test]
fn get_matches() {
    let result = client().get_matches();

    assert!(result.is_ok());
}

#[test]
fn get_standings() {
    let result = client().get_standings(1);

    assert!(result.is_ok());
}
