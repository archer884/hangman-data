use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel;
use models::{Game, NewGame};
use schema::games::dsl;
use schema::games;
use service::{ServiceConnection, ServiceError, ServiceResult};

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
    fn by_id(&self, id: i64) -> ServiceResult<Game>;
    fn create_game<'a, T: AsRef<NewGame<'a>>>(&self, game: T) -> ServiceResult<()>;
    fn update(&self, id: i64, state: &str, outcome: &str) -> ServiceResult<()>;
    fn update_outcome(&self, id: i64, outcome: &str) -> ServiceResult<()>;
    fn update_state(&self, id: i64, state: &str) -> ServiceResult<()>;    
}

impl GameService for PgGameService {
    fn by_id(&self, id: i64) -> ServiceResult<Game> {
        match dsl::games.filter(dsl::id.eq(id)).first(&*self.connection) {
            Ok(game) => Ok(game),
            Err(DieselError::NotFound) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Other(box e)),
        }
    }

    fn create_game<'a, T: AsRef<NewGame<'a>>>(&self, game: T) -> ServiceResult<()> {
        match diesel::insert(game.as_ref()).into(games::table).execute(&*self.connection) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InsertFailed),
        }
    }
    
    fn update(&self, id: i64, game_state: &str, outcome: &str) -> ServiceResult<()> {
        let result = diesel::update(dsl::games.find(id))
            .set((
                dsl::game_state.eq(game_state),
                dsl::outcome.eq(outcome),
            ))
            .execute(&*self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::UpdateFailed),
        }
    }
    
    fn update_outcome(&self, id: i64, outcome: &str) -> ServiceResult<()> {
        let result = diesel::update(dsl::games.find(id))
            .set(dsl::outcome.eq(outcome))
            .execute(&*self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::UpdateFailed),
        }
    }
    
    fn update_state(&self, id: i64, state: &str) -> ServiceResult<()> {
        let result = diesel::update(dsl::games.find(id))
            .set(dsl::game_state.eq(state))
            .execute(&*self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::UpdateFailed),
        }
    }
}