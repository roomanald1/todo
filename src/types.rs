use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TODO {
    pub description: String,
    pub added_on: String,
    pub completed: bool,
    pub id: Option<u64>,
    pub user_id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct State {
    pub(crate) items : Vec<TODO>
}