use schema::{games, tokens};

#[derive(Debug, Queryable)]
pub struct Token {
    pub id: i64,
    pub token: String,
}

#[derive(Debug)]
#[insertable_into(tokens)]
pub struct NewToken<'a> {
    pub token: &'a str,
}

impl<'a> NewToken<'a> {
    pub fn new(token: &'a str) -> NewToken<'a> {
        NewToken {
            token: token
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Game {
    pub id: i64,
    pub token_id: i64,
    pub game_state: String,
    pub outcome: String,
}

#[derive(Debug)]
#[insertable_into(games)]
pub struct NewGame<'a> {
    pub token_id: i64,
    pub game_state: &'a str,
    pub outcome: &'a str,
}