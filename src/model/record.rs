use postgres::rows::Row;

#[derive(Debug)]
pub struct Record {
    pub token_id: i64,
    pub token: String,
    pub wins: i64,
    pub losses: i64,
}

impl<'a> From<Row<'a>> for Record {
    fn from(row: Row) -> Record {
        Record {
            token_id: row.get(0),
            token: row.get(1),
            wins: row.get(2),
            losses: row.get(3),
        }
    }
}