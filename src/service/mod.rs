use std::error::Error;
use std::fmt;
use postgres::error::Error as PgError;
use postgres::rows::{Row, Rows};
use r2d2::PooledConnection;
use r2d2_postgres;

mod connection;
mod game;
mod page;
mod token;

pub use service::connection::{ConnectionService, PgConnectionService};
pub use service::game::{GameService, PgGameService};
pub use service::page::Page;
pub use service::token::{TokenService, PgTokenService};

pub type ConnectionManager = r2d2_postgres::PostgresConnectionManager;
pub type ServiceConnection = PooledConnection<ConnectionManager>;
pub type ServiceResult<T> = Result<T, ServiceError>;

pub enum ServiceError {
    NotFound,
    BadSchema,
    DatabaseFailure(Box<Error>),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ServiceError::NotFound => write!(f, "Not found"),
            &ServiceError::BadSchema => write!(f, "Unable to convert pg type to rust type"),
            &ServiceError::DatabaseFailure(ref e) => write!(f, "Database failure: {:?}", e),
        }
    }
}

impl From<PgError> for ServiceError {
    fn from(error: PgError) -> Self {
        match error {
            PgError::Db(e) => ServiceError::DatabaseFailure(e),
            PgError::Io(e) => ServiceError::DatabaseFailure(box e),
            PgError::Conversion(_) => ServiceError::BadSchema,
        }
    }
}

pub trait IntoModel<'a> {
    fn single<T: From<Row<'a>>>(&'a self) -> ServiceResult<T>;
    fn multiple<T: From<Row<'a>>>(&'a self) -> Vec<T>;
}

impl<'a> IntoModel<'a> for Rows<'a> {
    fn single<T: From<Row<'a>>>(&'a self) -> ServiceResult<T> {
        self.iter().map(Row::into).next().ok_or(ServiceError::NotFound)
    }
    
    fn multiple<T: From<Row<'a>>>(&'a self) -> Vec<T> {
        self.iter().map(Row::into).collect()
    }
}