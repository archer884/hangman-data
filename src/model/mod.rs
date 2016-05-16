use postgres::types::Type;

mod difficulty;
mod game;
mod outcome;
mod record;
mod token;

pub use model::difficulty::Difficulty;

pub use model::game::{
    Game,
    CreateGame,
    UpdateGame,
};

pub use model::outcome::Outcome;
pub use model::record::Record;

pub use model::token::{
    Token,
    CreateToken,
};

fn accepts_strings(ty: &Type) -> bool {
    match *ty {
        Type::Varchar | Type::Text | Type::Bpchar | Type::Name => true,
        Type::Other(ref u) if u.name() == "citext" => true,
        _ => false,
    }
}