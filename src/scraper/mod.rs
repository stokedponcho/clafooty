use super::domain;

pub mod ligue2;

pub struct Client {}

impl Client {
    pub fn get_standings(competition_id: u16) -> Option<domain::StandingCollection> {
        match competition_id {
            1602 => Some(ligue2::get_standings()),
            _ => None,
        }
    }
}
