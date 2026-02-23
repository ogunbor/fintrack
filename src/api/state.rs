use crate::configuration::DbPool;

pub struct AppState {
    pub pool: DbPool,
    pub jwt_secret: String,
}
