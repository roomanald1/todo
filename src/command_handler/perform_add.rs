use crate::store::database;
use crate::types;
use crate::command_handler::commands::CommandResult;
use chrono::Utc;
use tokio::runtime::Runtime;
use tokio_postgres::Client;
use crate::command_handler::commands;

pub(crate) async fn perform_add(description: String, client: &Client) -> commands::CommandResult {
    let item = types::Todo {
        id: None,
        user_id: String::from("user_123"),
        description,
        added_on: Utc::now().to_string(),
        completed: false
    };


    let id = database::upsert_item(client, item).await;
    match id {
        Ok(id) => CommandResult::Success(format!("Task with ID={} added successfully.", id)),
        Err(e) => CommandResult::Failure(e.to_string())
    }
}

