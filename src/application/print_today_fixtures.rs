use football_data::client::Client;

use crate::{application::mappers::map_match, domain};

pub fn print_today_fixtures(client: Client) {
    let fixtures_dto = client
        .get_fixtures()
        .unwrap_or_else(|error| panic!("{}", error.message));
    let matches = fixtures_dto
        .matches
        .iter()
        .map(|dto| -> domain::Match { map_match(dto) })
        .collect();
    let fixtures = domain::FixtureCollection {
        count: fixtures_dto.count,
        matches,
    };

    if fixtures.count > 0 {
        println!("{}", fixtures);
    } else {
        println!("No matches today.");
    }
}
