use crate::setting::Setting;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;

pub async fn conn_getting(setting: Arc<Setting>) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(setting.database.url_getting().as_str())
        .await?;

    Ok(pool)
}
