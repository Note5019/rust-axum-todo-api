use chrono::NaiveDateTime;

use crate::{time_helper::IntoTimerHelperShared, todo::model::TodoModel};

#[derive(sqlx::FromRow)]
pub struct Todos {
    pub id: Option<i32>, // nullable, Can be None
    pub topic: String,
    pub completed: bool,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Todos {
    pub fn new(topic: String, time_helper: IntoTimerHelperShared) -> Self {
        Self {
            id: None,
            topic,
            completed: false,
            completed_at: None,
            created_at: time_helper.now(),
            updated_at: time_helper.now(),
        }
    }

    pub fn to_model(&self) -> TodoModel {
        TodoModel {
            id: self.id.unwrap(),
            topic: self.topic.clone(),
            completed: self.completed,
        }
    }
}
