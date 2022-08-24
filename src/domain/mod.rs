use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Competition {
    pub id: u16,
    pub name: String,
    pub current_match_day: Option<u8>,
}

#[derive(Debug)]
pub struct FixtureCollection {
    pub count: u8,
    pub competition: Competition,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
pub enum MatchStatus {
    Scheduled,
    InPlay,
    Paused,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug)]
pub struct Match {
    pub utc_date: DateTime<Utc>,
    pub status: Option<MatchStatus>,
    pub home_team: String,
    pub away_team: String,
    pub score: ScoreCard,
}

#[derive(Debug)]
pub struct ScoreCard {
    pub winner: Option<String>,
    pub half_time: Score,
    pub full_time: Score,
}

#[derive(Debug)]
pub struct Score {
    pub home_team: Option<u8>,
    pub away_team: Option<u8>,
}

#[derive(Debug)]
pub struct StandingCollection {
    pub competition: self::Competition,
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
    pub points: u8,
    pub goals_for: u8,
    pub goals_against: u8,
    pub goal_difference: i8,
}
