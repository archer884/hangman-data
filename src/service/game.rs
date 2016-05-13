use model::{Game, CreateGame, UpdateGame};
use service::{IntoModel, Page, ServiceConnection, ServiceResult};

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
    fn page(&self, token: &str, page: &Page) -> ServiceResult<Vec<Game>>;
    fn latest(&self, token: &str) -> ServiceResult<Game>;
    fn create_game<T>(&self, game: T) -> ServiceResult<u64>
        where T: CreateGame;
    fn update<T>(&self, game: T) -> ServiceResult<u64>
        where T: UpdateGame;
    fn update_outcome(&self, id: i64, outcome: &str) -> ServiceResult<u64>;
    fn update_state(&self, id: i64, state: &str) -> ServiceResult<u64>;
}

impl GameService for PgGameService {
    fn by_id(&self, id: i64) -> ServiceResult<Game> {
        let sql = include_str!("../../sql/game/by_id.sql");
        self.connection.query(sql, &[&id])?.single()
    }
    
    fn page(&self, token: &str, page: &Page) -> ServiceResult<Vec<Game>> {
        let sql = include_str!("../../sql/game/page.sql");
        Ok(self.connection.query(sql, &[
            &token,
            &page.skip(),
            &page.take(),
        ])?.multiple())
    }

    fn latest(&self, token: &str) -> ServiceResult<Game> {
        let sql = include_str!("../../sql/game/latest.sql");
        self.connection.query(sql, &[&token])?.single()
    }

    fn create_game<T>(&self, game: T) -> ServiceResult<u64>
        where T: CreateGame
    {
        let sql = include_str!("../../sql/game/create.sql");
        Ok(self.connection.execute(sql, &[
            &game.token_id(),
            &game.state(),
            &game.outcome(),
        ])?)
    }

    fn update<T>(&self, game: T) -> ServiceResult<u64>
        where T: UpdateGame
    {
        let sql = include_str!("../../sql/game/update.sql");
        Ok(self.connection.execute(sql, &[
            &game.id(),
            &game.state(),
            &game.outcome(),
        ])?)
    }

    fn update_outcome(&self, id: i64, outcome: &str) -> ServiceResult<u64> {
        let sql = include_str!("../../sql/game/update_outcome.sql");
        Ok(self.connection.execute(sql, &[
            &id,
            &outcome
        ])?)
    }

    fn update_state(&self, id: i64, state: &str) -> ServiceResult<u64> {
        let sql = include_str!("../../sql/game/update_state.sql");
        Ok(self.connection.execute(sql, &[
            &id,
            &state
        ])?)
    }
}
