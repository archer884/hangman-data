use postgres::rows::Row;
use model::{Difficulty, Outcome};

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub token_id: i64,
    pub state: String,
    pub difficulty: Difficulty,
    pub outcome: Outcome,
}

pub trait CreateGame {
    fn token(&self) -> &str;
    fn state(&self) -> &str;
    fn difficulty(&self) -> &str;
}

impl<T: AsRef<str>> CreateGame for (T, T, T) {
    fn token(&self) -> &str {
        self.0.as_ref()
    }
    
    fn state(&self) -> &str {
        self.1.as_ref()
    }
    
    fn difficulty(&self) -> &str {
        self.2.as_ref()
    }
}

pub trait UpdateGame {
    fn id(&self) -> i64;
    fn state(&self) -> &str;
    fn outcome(&self) -> &str;
}

impl<T: AsRef<str>> UpdateGame for (i64, T, T) {
    fn id(&self) -> i64 {
        self.0
    }
    
    fn state(&self) -> &str {
        self.1.as_ref()
    }
    
    fn outcome(&self) -> &str {
        self.2.as_ref()
    }
}

impl<'a> From<Row<'a>> for Game {
    fn from(row: Row) -> Game {
        Game {
            id: row.get(0),
            token_id: row.get(1),
            state: row.get(2),
            difficulty: row.get(3),
            outcome: row.get(4),
        }
    }
}
