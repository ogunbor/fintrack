use crate::configuration::DbPool;

pub struct AppState {
    pub pool: DbPool,
}