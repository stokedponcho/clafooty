use football_data::client::Client as FootballData;

use super::mappers::map_competition;
use crate::application::mappers::map_fixtures;
use crate::configuration::Configuration;
use crate::domain::FixtureCollection;
use crate::scraper::Client as Scraper;

trait GetFixtures {
    fn execute(&self, competition_id: u16, matchday: Option<u8>) -> Vec<FixtureCollection>;
}

pub fn print_current_fixtures(
    configuration: Configuration,
    competition_ids: Vec<u16>,
    matchday: Option<u8>,
) {
    let providers: Vec<Box<dyn GetFixtures>> = vec![
        Box::new(Scraper {}),
        Box::new(FootballData::new(&configuration.token, None).unwrap()),
    ];

    competition_ids
        .iter()
        .flat_map(|competition_id| {
            for provider in providers.as_slice() {
                let collections = provider.execute(*competition_id, matchday);

                if !collections.is_empty() {
                    return Some(collections);
                }
            }

            None
        })
        .flatten()
        .for_each(|standings| {
            println!("{}", standings);
        });
}

impl GetFixtures for Scraper {
    fn execute(&self, competition_id: u16, matchday: Option<u8>) -> Vec<FixtureCollection> {
        match Scraper::get_fixtures(competition_id, matchday) {
            Some(collection) => vec![collection],
            None => vec![],
        }
    }
}

impl GetFixtures for FootballData {
    fn execute(&self, competition_id: u16, matchday: Option<u8>) -> Vec<FixtureCollection> {
        let matchday = match matchday {
            Some(_) => matchday,
            None => match self.get_competition(competition_id) {
                Ok(competition) => Some(map_competition(&competition).current_matchday.unwrap()),
                Err(football_data::dtos::ErrorMessage {
                    error_code: 404,
                    message: _,
                }) => None,
                Err(error) => panic!("{}", error.message),
            },
        };

        match matchday {
            Some(matchday) => match self.get_competition_matches(competition_id, matchday) {
                Ok(collection) => map_fixtures(collection),
                Err(football_data::dtos::ErrorMessage {
                    error_code: 404,
                    message: _,
                }) => vec![],
                Err(error) => panic!("{}", error.message),
            },
            None => vec![],
        }
    }
}
