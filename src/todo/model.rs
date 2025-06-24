use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTodo {
    pub topic: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoModel {
    pub id: i32,
    pub topic: String,
    pub completed: bool,
    // pub completed_at: Option<NaiveDateTime>,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub topic: Option<String>,
    pub completed: Option<bool>,
}
