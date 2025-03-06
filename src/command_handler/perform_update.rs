use crate::store::database;
use crate::command_handler::commands::CommandResult;
use tokio_postgres::Client;
use tracing::instrument;
use crate::command_handler::commands;
use serde_json;
use crate::types::Todo;

#[instrument]
pub(crate) async fn perform_update(item: String, client: &Client, user: String) -> commands::CommandResult {
    match serde_json::from_str(&item) {
        Ok(value) => {
            let v: Todo = value;

            let id = database::upsert_item(client, v).await;
            match id {
                Ok(id) => CommandResult::Success(format!("Task with ID={} added successfully.", id)),
                Err(e) => CommandResult::Failure(e.to_string())
            }
        },
        Err(e) => {
            CommandResult::Failure(format!("Failed to deserialize item {}", e))
        }
    }
}

