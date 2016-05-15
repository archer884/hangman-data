extern crate hangman_data;

use hangman_data::service::{ConnectionService, PgConnectionService, TokenService};

fn main() {
    let mut service = PgConnectionService::new();

    match std::env::args().nth(1).and_then(|n| n.parse::<i64>().ok()) {
        None => println!("Enter an ID"),
        Some(id) => match service.tokens().by_id(id) {
            Err(e) => println!("{}", e),
            Ok(token) => println!("{:?}", token),
        }
    }
}