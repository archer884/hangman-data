#![feature(box_syntax, custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod error;
pub mod models;
pub mod schema;
pub mod service;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use error::DbError;
use models::{Game, NewGame, NewToken, Token};

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(
        &format!("Error connecting to {}", database_url)
    )
}

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

pub fn get_token(conn: &PgConnection, token_id: i64) -> Result<Token, DbError> {
    use schema::tokens::dsl::*;
    tokens.filter(id.eq(token_id)).first(conn).map_err(|_| DbError::NotFound)
}

pub fn get_game(conn: &PgConnection, game_id: i64) -> Result<Game, DbError> {
    use schema::games::dsl::*;
    games.filter(id.eq(game_id)).first(conn).map_err(|_| DbError::NotFound)
}