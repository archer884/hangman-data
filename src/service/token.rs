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
    fn create<T>(&self, token: T) -> ServiceResult<u64>
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

    fn create<T>(&self, token: T) -> ServiceResult<u64>
        where T: CreateToken
    {
        let sql = include_str!("../../sql/token/create.sql");
        Ok(self.connection.execute(sql, &[
            &token.token()
        ])?)
    }
}

// Note that these function as integration tests, not unit tests
#[cfg(test)]
mod tests {
    use service::{PgConnectionService, ConnectionService, TokenService};
    
    #[test]
    fn by_id() {
        get_service().tokens().by_id(1).unwrap();
    }
    
    #[test]
    fn by_token() {
        get_service().tokens().by_token("sanford_and_son").unwrap();
    }
    
    #[test]
    fn record() {
        get_service().tokens().record("sanford_and_son").unwrap();
    }
    
    #[test]
    #[ignore]
    fn create() {
        let rows_affected = get_service().tokens().create("mike_and_ike").unwrap();
        
        assert_eq!(1, rows_affected);
    }
    
    fn get_service() -> PgConnectionService {
        PgConnectionService::new()
    }
}