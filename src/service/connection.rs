use std::env;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;
use service::{GameService, TokenService, PgGameService, PgTokenService};

pub struct PgConnectionService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgConnectionService {
    pub fn new() -> PgConnectionService {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL must be set"
        );
        
        let pool = Pool::new(
            Config::default(),
            ConnectionManager::new(database_url)
        ).expect("failed to initialize pool");
        
        PgConnectionService {
            pool: pool,
        }
    }
}

pub trait ConnectionService {
    type GameService: GameService;
    type TokenService: TokenService;
    
    fn tokens(&mut self) -> Self::TokenService;
    fn games(&mut self) -> Self::GameService; 
}

impl ConnectionService for PgConnectionService {
    type TokenService = PgTokenService;
    type GameService = PgGameService;
    
    fn tokens(&mut self) -> Self::TokenService {
        PgTokenService::new(self.pool.get().expect("unable to get connection"))
    }
    
    fn games(&mut self) -> Self::GameService {
        PgGameService::new(self.pool.get().expect("unable to get connection"))
    }
}