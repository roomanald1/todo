use tokio_postgres::Client;
use commands::CommandResult::Success;
use crate::command_handler::commands;
use crate::store::database;
use commands::CommandResult::Failure;

use super::commands::CommandResult;

pub(crate) async fn perform_done_toggle(item: String, done: bool, client: &Client, user: String) -> commands::CommandResult{
    match item.parse::<i64>() {
        Ok(id) => mark_item(client, id, done, user).await,
        Err(_) => Failure("Invalid item ID".to_string()),
    }
}

async fn mark_item(client: &Client, id: i64, done: bool, user: String) -> CommandResult {
    match database::mark_item(client, id, done, user).await{
        Ok(_) => Success(format!("Task with ID={} updated successfully.",id)),
        Err(err) => Failure(err.to_string()),
    }
}