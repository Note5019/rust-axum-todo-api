use std::sync::Arc;

use async_trait::async_trait;
use crate::todo::entities::Todos;
use sqlx::PgPool;
use tracing::error;

pub type SharedTodoRepository = Arc<dyn TodoMethod + Send + Sync>;

#[async_trait]
pub trait TodoMethod {
    async fn add_todo(&self, todo: Todos) -> Result<Todos, sqlx::Error>;
}

pub struct TodoRepository {
    db: PgPool,
}

impl TodoRepository {
    pub fn creation(db: PgPool) -> SharedTodoRepository {
        Arc::new(Self { db })
    }
}

#[async_trait]
impl TodoMethod for TodoRepository {
    async fn add_todo(&self, todo: Todos) -> Result<Todos, sqlx::Error> {
        let res: Todos = match sqlx::query_as::<_, Todos>(
            "INSERT INTO todos (topic, completed, completed_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *;"
        )
        .bind(todo.topic)
        .bind(todo.completed)
        .bind(todo.completed_at)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .fetch_one(&self.db)
        .await {
            Ok(todo) => todo,
            Err(e) => {
                error!("Failed to insert todo: {:?}", e);
                return Err(e);
            }
        };

        match res.id {
            Some(_) => Ok(res),
            None =>{
                error!("Failed to insert todo:");
                return Err(sqlx::Error::RowNotFound);
            }
        }
    }
}
