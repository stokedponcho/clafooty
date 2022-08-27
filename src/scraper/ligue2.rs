use crate::domain;
use reqwest::Url;
use scraper::{Html, Selector};

pub fn get_standings() -> domain::StandingCollection {
    let url = Url::parse("https://www.ligue2.fr/classement").unwrap();
    let response = reqwest::blocking::get(url).unwrap();
    let body = match response.text() {
        Ok(value) => value,
        _ => panic!("{}", "No response"),
    };
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".classement-table-body .GeneralStats-row").unwrap();
    let standings = document.select(&selector).into_iter().map(|element| {
        let next = |selector: &mut scraper::html::Select| selector.next().unwrap().inner_html();
        let as_uint = |value: String| -> u8 { value.parse().expect("Wanted a number") };
        let as_int = |value: String| -> i8 { value.parse().expect("Wanted a number") };
        let as_name = |value: String| -> String {
            let fragment = Html::parse_fragment(&value);
            let selector = Selector::parse(".GeneralStats-clubName").unwrap();

            fragment.select(&selector).next().unwrap().inner_html()
        };
        let fragment = Html::parse_fragment(&element.inner_html());
        let selector = Selector::parse(".GeneralStats-item").unwrap();
        let mut input = fragment.select(&selector);

        domain::Standing {
            position: as_uint(next(&mut input)),
            team: as_name(next(&mut input)),
            points: as_int(next(&mut input)),
            played_games: as_uint(next(&mut input)),
            won: as_uint(next(&mut input)),
            draw: as_uint(next(&mut input)),
            lost: as_uint(next(&mut input)),
            goals_for: as_uint(next(&mut input)),
            goals_against: as_uint(next(&mut input)),
            goal_difference: as_int(next(&mut input)),
        }
    });

    domain::StandingCollection {
        competition: domain::Competition {
            id: 0,
            name: "Ligue 2".to_string(),
            current_matchday: None,
        },
        table: standings.collect(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        println!("{}", get_standings());
    }
}
