extern crate hangman_data;

fn main() {
    println!("{:?}", hangman_data::get_token(
        &hangman_data::connect(),
        std::env::args().nth(1).and_then(|n| 
            n.parse::<i64>().ok()
        ).expect("invalid id"),
    ))
}