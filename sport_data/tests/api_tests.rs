use reqwest::Url;

use sport_data::client::Client;

fn client() -> Client {
    Client::new(
        "SOME-AUTH-TOKEN",
        Some(Url::parse("http://localhost:8589/soccer/").unwrap()),
    )
    .unwrap()
}

#[test]
fn get_seasons_by_league() {
    let result = client().get_seasons_by_league(1);

    println!("{:?}", &result.as_ref().err());

    assert!(result.is_ok());
}
