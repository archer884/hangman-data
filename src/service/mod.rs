use std::error::Error;
use std::fmt;
use diesel::pg::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

mod connection;
mod game;
mod token;

pub use service::connection::{ConnectionService, PgConnectionService};
pub use service::game::{GameService, PgGameService};
pub use service::token::{TokenService, PgTokenService};

pub type ServiceConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    InsertFailed,
    UpdateFailed,
    NotFound,
    Other(Box<Error>),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ServiceError::InsertFailed => write!(f, "Insert failed"),
            &ServiceError::UpdateFailed => write!(f, "Update failed"),
            &ServiceError::NotFound => write!(f, "Not found"),
            &ServiceError::Other(ref e) => write!(f, "{}", e),
        }
    }
}