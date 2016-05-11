#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{Game, NewGame, NewToken, Token};

#[derive(Debug)]
pub enum DbError {
    NotFound
}

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(
        &format!("Error connecting to {}", database_url)
    )
}

// I'm going to return a bool here, indicating success or failure, until I know more about
// just what it is I actually want.
pub fn create_token<'a, T>(conn: &PgConnection, token: T) -> bool 
    where T: AsRef<NewToken<'a>>
{
    use schema::tokens;
    diesel::insert(token.as_ref()).into(tokens::table).execute(conn).is_ok()
}

pub fn create_game<'a, T>(conn: &PgConnection, game: T) -> bool
    where T: AsRef<NewGame<'a>>
{
    use schema::games;
    diesel::insert(game.as_ref()).into(games::table).execute(conn).is_ok()
}

pub fn get_token(conn: &PgConnection, id: i64) -> Result<Token, DbError> {
    use schema::tokens::dsl::*;
    tokens.find(id).first(conn).map_err(|_| DbError::NotFound)
}

pub fn get_game(conn: &PgConnection, id: i64) -> Result<Game, DbError> {
    use schema::games::dsl::*;
    games.find(id).first(conn).map_err(|_| DbError::NotFound)
}