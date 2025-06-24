use sqlx::Postgres;

use crate::time_helper::IntoTimerHelperShared;

pub struct AppState {
    pub db: sqlx::Pool<Postgres>,
    pub time_helper: IntoTimerHelperShared,
}