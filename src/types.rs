use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Todo {
    pub description: String,
    pub added_on: String,
    pub completed: bool,
    pub id: Option<i64>,
    pub is_deleted: Option<bool>,
    pub last_updated: Option<String>,
    pub user_id: String,
    pub detail: Option<String>,
    pub due: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UpdateAction {
    Add,
    Remove,
    Update
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TodoUpdate {
    pub action : UpdateAction,
    pub id : i64,
    pub data: Option<Todo>
}