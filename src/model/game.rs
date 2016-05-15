use postgres::rows::Row;

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub token_id: i64,
    pub state: String,
    pub outcome: String,
}

pub trait CreateGame {
    fn token_id(&self) -> i64;
    fn state(&self) -> &str;
    fn outcome(&self) -> &str;
}

impl<T: AsRef<str>> CreateGame for (i64, T, T) {
    fn token_id(&self) -> i64 {
        self.0
    }
    
    fn state(&self) -> &str {
        self.1.as_ref()
    }
    
    fn outcome(&self) -> &str {
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
            outcome: row.get(3),
        }
    }
}
