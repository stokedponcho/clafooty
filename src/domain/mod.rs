use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Competition {
    pub id: u16,
    pub name: String,
    pub current_matchday: Option<u8>,
}

#[derive(Debug, PartialEq)]
pub struct FixtureCollection {
    pub competition: Competition,
    pub matches: Vec<Match>,
    pub matchday: Option<u8>,
    pub stage: String,
}

#[derive(Debug, PartialEq)]
pub enum MatchStatus {
    Scheduled,
    InPlay,
    Paused,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug, PartialEq)]
pub struct Match {
    pub date: Option<NaiveDate>,
    pub datetime: Option<DateTime<Utc>>,
    pub status: Option<MatchStatus>,
    pub home_team: String,
    pub away_team: String,
    pub score: ScoreCard,
}

#[derive(Debug, PartialEq)]
pub struct ScoreCard {
    pub winner: Option<String>,
    pub half_time: Option<Score>,
    pub full_time: Option<Score>,
}

#[derive(Debug, PartialEq)]
pub struct Score {
    pub home_team: u8,
    pub away_team: u8,
}

#[derive(Debug)]
pub struct StandingCollection {
    pub competition: Competition,
    pub table: Vec<Standing>,
}

#[derive(Debug)]
pub struct Standing {
    pub position: u8,
    pub team: String,
    pub played_games: u8,
    pub won: u8,
    pub draw: u8,
    pub lost: u8,
    pub points: i8,
    pub goals_for: u8,
    pub goals_against: u8,
    pub goal_difference: i8,
}
