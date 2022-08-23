use football_data::dtos;

use crate::domain;

pub fn map_match(dto: &dtos::Match) -> domain::Match {
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
