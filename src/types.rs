use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Todo {
    pub description: String,
    pub added_on: String,
    pub completed: bool,
    pub id: Option<i64>,
    pub user_id: String,
    pub detail: Option<String>,
    pub due: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct State {
    pub(crate) items : Vec<Todo>
}