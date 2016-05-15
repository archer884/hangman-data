extern crate hangman_data;

use hangman_data::service::{ConnectionService, Page, PgConnectionService, GameService};

fn main() {
    let mut service = PgConnectionService::new();

    match std::env::args().nth(1).and_then(|n| n.parse::<i64>().ok()) {
        None => println!("Enter page length"),
        Some(length) => match service.games().page("sanford_and_son", &Page::new(length)) {
            Err(e) => println!("{}", e),
            Ok(games) => println!("{:?}", games),
        }
    }
}