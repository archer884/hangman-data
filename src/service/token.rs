use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel;
use models::{Token, NewToken};
use schema::tokens::dsl;
use schema::tokens;
use service::{ServiceConnection, ServiceError, ServiceResult};

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
    fn create_token<'a, T>(&self, token: T) -> ServiceResult<()>
        where T: AsRef<NewToken<'a>>;
}

impl TokenService for PgTokenService {
    fn by_id(&self, id: i64) -> ServiceResult<Token> {
        match dsl::tokens.filter(dsl::id.eq(id)).first(&*self.connection) {
            Ok(token) => Ok(token),
            Err(DieselError::NotFound) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Other(box e)),
        }
    }
    
    fn by_token(&self, token: &str) -> ServiceResult<Token> {
        match dsl::tokens.filter(dsl::token.eq(token)).first(&*self.connection) {
            Ok(token) => Ok(token),
            Err(DieselError::NotFound) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Other(box e)),
        }
    }
    
    fn create_token<'a, T>(&self, token: T) -> ServiceResult<()>
        where T: AsRef<NewToken<'a>>
    {
        match diesel::insert(token.as_ref()).into(tokens::table).execute(&*self.connection) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InsertFailed),
        }
    }
}