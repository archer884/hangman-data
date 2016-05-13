use model::{CreateToken, Record, Token};
use service::{IntoModel, ServiceConnection, ServiceResult};

pub struct PgTokenService {
    connection: ServiceConnection
}

impl PgTokenService {
    pub fn new(connection: ServiceConnection) -> PgTokenService {
        PgTokenService {
            connection: connection
        }
    }
}

pub trait TokenService {
    fn by_id(&self, id: i64) -> ServiceResult<Token>;
    fn by_token(&self, token: &str) -> ServiceResult<Token>;
    fn record(&self, token: &str) -> ServiceResult<Record>;
    fn create_token<T>(&self, token: T) -> ServiceResult<u64>
        where T: CreateToken;
}

impl TokenService for PgTokenService {
    fn by_id(&self, id: i64) -> ServiceResult<Token> {
        let sql = include_str!("../../sql/token/by_id.sql");
        self.connection.query(sql, &[&id])?.single()
    }

    fn by_token(&self, token: &str) -> ServiceResult<Token> {
        let sql = include_str!("../../sql/token/by_token.sql");
        self.connection.query(sql, &[&token])?.single()
    }

    fn record(&self, token: &str) -> ServiceResult<Record> {
        let sql = include_str!("../../sql/token/record.sql");
        self.connection.query(sql, &[&token])?.single()
    }

    fn create_token<T>(&self, token: T) -> ServiceResult<u64>
        where T: CreateToken
    {
        let sql = include_str!("../../sql/token/create.sql");
        Ok(self.connection.execute(sql, &[
            &token.token()
        ])?)
    }
}
