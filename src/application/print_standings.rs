use football_data::client::Client;

use crate::application::mappers::map_competition;
use crate::domain::{Standing, StandingCollection};

pub fn print_standings(client: Client, competition_id: u16) {
    let standings_dto = client
        .get_standings(competition_id)
        .unwrap_or_else(|error| panic!("{}", error.message));
    let standings = StandingCollection {
        competition: map_competition(&standings_dto.competition),
        table: standings_dto.standings[0]
            .table
            .iter()
            .map(|dto| Standing {
                position: dto.position,
                team: dto.team.name.clone(),
                played_games: dto.played_games,
                won: dto.won,
                draw: dto.draw,
                lost: dto.lost,
                points: dto.points,
                goals_for: dto.goals_for,
                goals_against: dto.goals_against,
                goal_difference: dto.goal_difference,
            })
            .collect(),
    };

    println!("{}", standings);
}
