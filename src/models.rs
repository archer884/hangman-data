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