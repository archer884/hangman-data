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
    fn create<T>(&self, game: T) -> ServiceResult<i64>
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

    fn create<T>(&self, game: T) -> ServiceResult<i64>
        where T: CreateGame
    {
        let sql = include_str!("../../sql/game/create.sql");
        self.connection.query(sql, &[
            &game.token_id(),
            &game.state(),
            &game.difficulty(),
        ])?.id()
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

// Note that these function as integration tests, not unit tests
#[cfg(test)]
mod tests {
    use service::{Page, PgConnectionService, ConnectionService, GameService};
    
    #[test]
    fn by_id() {
        get_service().games().by_id(1).unwrap();
    }
    
    #[test]
    fn page() {
        let page = get_service().games().page(
            "sanford_and_son",
            &Page::new(0),
        ).unwrap();
        
        assert!(page.len() >= 1);
    }
    
    #[test]
    fn latest() {
        get_service().games().latest("sanford_and_son").unwrap();
    }
    
    #[test]
    #[ignore]
    fn create() {
        let rows_affected = get_service().games().create((
            1,
            "invalid",
            "Easy",
        )).unwrap();
        
        assert_eq!(1, rows_affected);
    }
    
    #[test]
    #[ignore]
    fn update() {
        let rows_affected = get_service().games().update((
            1,
            "Your mom!",
            "Win",
        )).unwrap();
        
        assert_eq!(1, rows_affected);
    }
    
    #[test]
    #[ignore]
    fn update_outcome() {
        let rows_affected = get_service().games().update_outcome(1, "Loss").unwrap();
        
        assert_eq!(1, rows_affected);
    }
    
    #[test]
    #[ignore]
    fn update_state() {
        let rows_affected = get_service().games().update_state(1, "still invalid").unwrap();
        
        assert_eq!(1, rows_affected);
    }
    
    fn get_service() -> PgConnectionService {
        PgConnectionService::new()
    }
}