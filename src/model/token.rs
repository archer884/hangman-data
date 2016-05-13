use postgres::rows::Row;

#[derive(Debug)]
pub struct Token {
    pub id: i64,
    pub token: String,
}

pub trait CreateToken {
    fn token(&self) -> &str;
}

impl<'a> From<Row<'a>> for Token {
    fn from(row: Row) -> Token {
        Token {
            id: row.get(0),
            token: row.get(1),
        }
    }
}