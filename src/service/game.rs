use service::ServiceConnection;

pub struct PgGameService {
    connection: ServiceConnection,
}

impl PgGameService {
    pub fn new(connection: ServiceConnection) -> PgGameService {
        PgGameService {
            connection: connection
        }
    }
}

pub trait GameService {
    
}

impl GameService for PgGameService {
    
}