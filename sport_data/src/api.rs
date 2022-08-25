use super::client::Client;
use super::dtos::{Collection, Error, Season};

impl Client {
    pub fn get_seasons_by_league(&self, league_id: u16) -> Result<Collection<Season>, Error> {
        self.get::<Collection<Season>>(&format!("seasons?league_id={}", league_id))
    }
}
