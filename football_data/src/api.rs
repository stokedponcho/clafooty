use crate::client::Client;
use crate::dtos::{Competition, ErrorMessage, MatchCollection, StandingCollection};

impl Client {
    pub fn get_competition(&self, competition_id: u16) -> Result<Competition, ErrorMessage> {
        self.get::<Competition>(&format!("competitions/{}", competition_id))
    }

    pub fn get_competition_matches(
        &self,
        competition_id: u16,
        matchday: u8,
    ) -> Result<MatchCollection, ErrorMessage> {
        self.get::<MatchCollection>(&format!(
            "competitions/{}/matches?matchday={}",
            competition_id, matchday
        ))
    }

    pub fn get_matches(&self) -> Result<MatchCollection, ErrorMessage> {
        self.get::<MatchCollection>("matches")
    }

    pub fn get_standings(&self, competition_id: u16) -> Result<StandingCollection, ErrorMessage> {
        self.get::<StandingCollection>(&format!("competitions/{}/standings", competition_id))
    }
}
