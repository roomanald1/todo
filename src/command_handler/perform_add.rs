use crate::store::database;
use crate::types;
use crate::command_handler::commands::CommandResult;
use chrono::Utc;
use tokio_postgres::Client;
use tracing::instrument;
use crate::command_handler::commands;

#[instrument]
pub(crate) async fn perform_add(description: String, client: &Client, user: String) -> commands::CommandResult {
    let item = types::Todo {
        id: None,
        user_id: user,
        description,
        added_on: Utc::now().to_string(),
        completed: false,
        due: None,
        detail: None
    };


    let id = database::upsert_item(client, item).await;
    match id {
        Ok(id) => CommandResult::Success(format!("Task with ID={} added successfully.", id)),
        Err(e) => CommandResult::Failure(e.to_string())
    }
}

