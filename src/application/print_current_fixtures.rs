use crate::{
    application::mappers::map_match,
    domain::{self, Competition},
};

pub fn print_current_fixtures(client: football_data::client::Client, competition_ids: Vec<u16>) {
    let competitions: Vec<domain::Competition> = competition_ids
        .iter()
        .map(|c| {
            client
                .get_competition(*c)
                .unwrap_or_else(|error| panic!("{}", error.message))
        })
        .map(|dto| domain::Competition {
            id: dto.id,
            name: dto.name,
            current_match_day: Some(dto.current_season.unwrap().current_matchday),
        })
        .collect();

    competitions.iter().for_each(|competition| {
        let dto = client
            .get_competition_matches(competition.id, competition.current_match_day.unwrap())
            .unwrap_or_else(|error| panic!("{}", error.message));
        let matches: Vec<domain::Match> = dto
            .matches
            .iter()
            .map(|dto| -> domain::Match { map_match(dto) })
            .collect();
        let collection = domain::FixtureCollection {
            count: dto.result_set.count as u8,
            matches,
            competition: Competition {
                current_match_day: None,
                id: 0,
                name: "".to_string(),
            },
        };

        println!("{}", competition.name);
        println!("{}", collection);
    });
}
