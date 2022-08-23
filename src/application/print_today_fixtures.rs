use football_data::client::Client;

use crate::{application::mappers::map_match, domain};

pub fn print_today_fixtures(client: Client) {
    let dto = client
        .get_matches()
        .unwrap_or_else(|error| panic!("{}", error.message));
    let matches = dto
        .matches
        .iter()
        .map(|dto| -> domain::Match { map_match(dto) })
        .collect();
    let fixtures = domain::FixtureCollection {
        count: dto.result_set.count as u8,
        matches,
    };

    if fixtures.count > 0 {
        println!("{}", fixtures);
    } else {
        println!("No matches today.");
    }
}
