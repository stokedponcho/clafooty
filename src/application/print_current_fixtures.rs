use crate::domain;

use football_data::dtos;

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
        let fixtures = client
            .get_competition_fixtures(competition.id, competition.current_match_day.unwrap())
            .unwrap_or_else(|error| panic!("{}", error.message));
        let matches: Vec<domain::Match> = fixtures
            .matches
            .iter()
            .map(|dto| -> domain::Match { map_match(dto) })
            .collect();

        let collection = domain::FixtureCollection {
            count: fixtures.count,
            matches,
        };

        println!("{}", competition.name);
        println!("{}", collection);
    });
}

fn map_match(dto: &dtos::Match) -> domain::Match {
    let map_score = |dto: &dtos::Score| domain::Score {
        home_team: dto.home_team,
        away_team: dto.away_team,
    };

    domain::Match {
        utc_date: dto.utc_date,
        status: match dto.status {
            Some(dtos::MatchStatus::FINISHED) => Some(domain::MatchStatus::Finished),
            Some(dtos::MatchStatus::IN_PLAY) => Some(domain::MatchStatus::InPlay),
            Some(dtos::MatchStatus::PAUSED) => Some(domain::MatchStatus::Paused),
            Some(dtos::MatchStatus::POSTPONED) => Some(domain::MatchStatus::Postponed),
            Some(dtos::MatchStatus::SCHEDULED) => Some(domain::MatchStatus::Scheduled),
            Some(dtos::MatchStatus::CANCELLED) => Some(domain::MatchStatus::Cancelled),
            _ => None,
        },
        home_team: dto.home_team.name.clone(),
        away_team: dto.away_team.name.clone(),
        score: domain::ScoreCard {
            winner: dto.score.winner.clone(),
            half_time: map_score(&dto.score.half_time),
            full_time: map_score(&dto.score.full_time),
        },
    }
}
