use chrono::{DateTime, NaiveDate};
use reqwest::Url;
use scraper::{Html, Selector};

use crate::domain;

pub fn get_fixtures(matchday: Option<u8>) -> domain::FixtureCollection {
    let url = Url::parse(&format!(
        "https://www.ligue2.fr/en/calendrier-resultats{}",
        match matchday {
            Some(value) => format!("?matchDay={}", value),
            None => "".to_string(),
        }
    ));
    let body = reqwest::blocking::get(url.unwrap())
        .expect("Could not complete request")
        .text()
        .expect("Could not get HTML body");

    parse_fixtures(&body)
}

fn parse_fixtures(body: &str) -> domain::FixtureCollection {
    let document = Html::parse_document(body);
    let days: Vec<String> = document
        .select(&selector(".calendar-widget-day"))
        .map(|element| element.inner_html())
        .collect();
    let results_selector = selector(".calendar-widget-container ul");
    let results = document.select(&results_selector).into_iter().enumerate();
    let fixtures: Vec<domain::Match> = results
        .map(|(index, ul)| {
            let fragment = Html::parse_fragment(&ul.inner_html());
            let match_selector = selector(".match-result");

            fragment
                .select(&match_selector)
                .into_iter()
                .map(|element| -> domain::Match { parse_match(&days[index], element) })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    domain::FixtureCollection {
        competition: domain::Competition {
            id: 1602,
            name: "Ligue 2".to_string(),
            current_matchday: attribute(
                single_node(&document, ".lfp-container"),
                "data-current-day",
            ),
        },
        matches: fixtures,
        matchday: attribute(single_node(&document, ".Scorebar"), "data-current-day"),
        stage: "Saison".to_string(),
    }
}

fn parse_match(date: &str, element: scraper::ElementRef) -> domain::Match {
    let fragment = Html::parse_fragment(&element.inner_html());

    let team_selector = selector(".club .calendarTeamNameDesktop");
    let mut input = fragment.select(&team_selector);
    let home_team = input.next().unwrap().inner_html().clone();
    let away_team = input.next().unwrap().inner_html().clone();

    let score_selector = selector(".result span > span");
    let mut input = fragment.select(&score_selector);
    let home_score = input.next().unwrap().inner_html().parse().ok();
    let _ = input.next().unwrap();
    let away_score = input.next().unwrap().inner_html().parse().ok();
    let score: Option<(u8, u8)> = match (home_score, away_score) {
        (Some(home), Some(away)) => Some((home, away)),
        _ => None,
    };

    let datetime = match score {
        Some(_) => None,
        None => {
            let time_selector = selector(".result span > span");
            let time_html = fragment
                .select(&time_selector)
                .into_iter()
                .map(|e| e.inner_html())
                .collect::<Vec<_>>()
                .join("");
            let time = time_html.split(":").collect::<Vec<_>>();
            let hours: String = time[0].parse().ok().unwrap();
            let minutes: String = time[1].parse().ok().unwrap();

            match DateTime::parse_from_str(
                &format!("{} {}:{} +0200", date, hours, minutes),
                "%A %_d %B %Y %H:%M %z",
            ) {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }
    };

    let date = match NaiveDate::parse_from_str(date, "%A %_d %B %Y") {
        Ok(value) => Some(value),
        Err(_) => None,
    };

    let status = match (score, date) {
        (Some(_), _) => Some({
            let status_selector = selector(".Icon-live");
            let mut input = fragment.select(&status_selector);
            match input.next() {
                Some(_) => domain::MatchStatus::InPlay,
                None => domain::MatchStatus::Finished,
            }
        }),
        (_, Some(_)) => Some(domain::MatchStatus::Scheduled),
        (None, None) => None,
    };

    domain::Match {
        date: date,
        datetime: match datetime {
            Some(datetime) => Some(DateTime::from(datetime)),
            None => None,
        },
        status,
        home_team,
        away_team,
        score: domain::ScoreCard {
            winner: None,
            half_time: None,
            full_time: match score {
                Some((home, away)) => Some(domain::Score {
                    home_team: home,
                    away_team: away,
                }),
                None => None,
            },
        },
    }
}

pub fn get_standings() -> domain::StandingCollection {
    let body = reqwest::blocking::get(Url::parse("https://www.ligue2.fr/classement").unwrap())
        .expect("Could not complete request")
        .text()
        .expect("Could not get HTML body");

    parse_standings(&body)
}

fn parse_standings(body: &str) -> domain::StandingCollection {
    let document = Html::parse_document(body);
    let standings = document
        .select(&selector(".classement-table-body .GeneralStats-row"))
        .into_iter()
        .map(|element| {
            let next = |selector: &mut scraper::html::Select| selector.next().unwrap().inner_html();
            let as_uint = |value: String| -> u8 { value.parse().expect("Wanted a number") };
            let as_int = |value: String| -> i8 { value.parse().expect("Wanted a number") };
            let as_name = |value: String| -> String {
                let fragment = Html::parse_fragment(&value);

                fragment
                    .select(&selector(".GeneralStats-clubName"))
                    .next()
                    .unwrap()
                    .inner_html()
            };
            let fragment = Html::parse_fragment(&element.inner_html());
            let row_selector = selector(".GeneralStats-item");
            let mut input = fragment.select(&row_selector);

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
        })
        .collect();

    domain::StandingCollection {
        competition: domain::Competition {
            id: 0,
            name: "Ligue 2".to_string(),
            current_matchday: None,
        },
        table: standings,
    }
}

fn selector(query: &str) -> Selector {
    Selector::parse(query).unwrap()
}

fn single_node<'a>(html: &'a Html, query: &str) -> Option<&'a scraper::node::Element> {
    Some(html.select(&selector(query)).next()?.value())
}

fn attribute<T: std::str::FromStr>(
    value: Option<&scraper::node::Element>,
    attribute: &str,
) -> Option<T> {
    value?.attr(attribute)?.parse().ok()
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_parse_fixtures() {
        todo!();
    }

    #[test]
    #[ignore]
    fn test_parse_match() {
        todo!();
    }

    #[test]
    #[ignore]
    fn test_parse_standings() {
        todo!();
    }
}
