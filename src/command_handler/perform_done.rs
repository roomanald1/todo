use tokio_postgres::Client;
use commands::CommandResult::Success;
use crate::command_handler::commands;
use crate::store::database;
use commands::CommandResult::Failure;

use super::commands::CommandResult;

pub(crate) async fn perform_done_toggle(item: String, done: bool, client: &Client) -> commands::CommandResult{
    match item.parse::<i64>() {
        Ok(id) => mark_item(client, id, done).await,
        Err(_) => Failure("Invalid item ID".to_string()),
    }
}

async fn mark_item(client: &Client, id: i64, done: bool) -> CommandResult {
    match database::mark_item(client, id, done).await{
        Ok(_) => Success(format!("Task with ID={} updated successfully.",id)),
        Err(err) => Failure(err.to_string()),
    }
}