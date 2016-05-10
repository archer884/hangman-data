#[derive(Queryable)]
pub struct Token {
    pub id: i64,
    pub token: String,
}

#[derive(Queryable)]
pub struct Game {
    pub id: i64,
    pub token_id: i64,
    pub game_state: String,
}