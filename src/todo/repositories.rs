use std::sync::Arc;

use crate::todo::entities::Todos;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

pub type SharedTodoRepository = Arc<dyn TodoMethod + Send + Sync>;

#[async_trait]
pub trait TodoMethod {
    async fn add_todo(&self, todo: Todos) -> Result<Todos, sqlx::Error>;
    async fn get_todos(&self) -> Result<Vec<Todos>, sqlx::Error>;
    async fn get_todo(&self, id: i32) -> Result<Todos, sqlx::Error>;
    async fn update_todo(&self, todo: Todos) -> Result<(), sqlx::Error>;
    async fn delete_todo(&self, id: i32) -> Result<(), sqlx::Error>;
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
            None => {
                error!("Failed to insert todo:");
                return Err(sqlx::Error::RowNotFound);
            }
        }
    }

    async fn get_todos(&self) -> Result<Vec<Todos>, sqlx::Error> {
        match sqlx::query_as::<_, Todos>("SELECT * FROM todos;")
            .fetch_all(&self.db)
            .await
        {
            Ok(todos) => Ok(todos),
            Err(e) => {
                error!("Failed to query: {:?}", e);
                return Err(e);
            }
        }
    }

    async fn get_todo(&self, id: i32) -> Result<Todos, sqlx::Error> {
        match sqlx::query_as::<_, Todos>("SELECT * FROM todos WHERE id = $1;")
            .bind(id)
            .fetch_one(&self.db)
            .await
        {
            Ok(todos) => Ok(todos),
            Err(e) => {
                error!("Failed to query: {:?}", e);
                return Err(e);
            }
        }
    }

    async fn update_todo(&self, todo: Todos) -> Result<(), sqlx::Error> {
        let res = match sqlx::query(
            "UPDATE todos SET
            topic = $1,
            completed = $2,
            updated_at = $3,
            completed_at = $4
            WHERE id = $5
        ",
        )
        .bind(todo.topic)
        .bind(todo.completed)
        .bind(todo.updated_at)
        .bind(todo.completed_at)
        .bind(todo.id)
        .execute(&self.db)
        .await
        {
            Ok(row) => row,
            Err(e) => {
                error!("Failed to update todo: {:?}", e);
                return Err(e);
            }
        };

        match res.rows_affected() {
            0 => {
                error!("Failed to update todo:");
                return Err(sqlx::Error::RowNotFound);
            }
            _ => Ok(()),
        }
    }

    async fn delete_todo(&self, id: i32) -> Result<(), sqlx::Error> {
        let res = match sqlx::query(
            "DELETE FROM todos 
            WHERE id = $1;
        ",
        )
        .bind(id)
        .execute(&self.db)
        .await
        {
            Ok(row) => row,
            Err(e) => {
                error!("Failed to update todo: {:?}", e);
                return Err(e);
            }
        };

        match res.rows_affected() {
            0 => {
                error!("Failed to update todo:");
                return Err(sqlx::Error::RowNotFound);
            }
            _ => Ok(()),
        }
    }
}
