use football_data::client::Client;

use crate::application::mappers::map_fixtures;

pub fn print_today_fixtures(client: Client) {
    let dto = client
        .get_matches()
        .unwrap_or_else(|error| panic!("{}", error.message));
    let fixtures = map_fixtures(dto);

    for f in fixtures.as_slice() {
        println!("{}", f);
    }

    if fixtures.len() == 0 {
        println!("");
        println!("No matches today.");
    }
}
