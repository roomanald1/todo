use tokio_postgres::Client;
use tracing::instrument;
use crate::store::database;
use crate::command_handler::commands;
use crate::command_handler::commands::CommandResult;

#[instrument]
pub(crate) async fn perform_remove(item: String, client: &Client, user: String) -> commands::CommandResult{

    let result = database::remove_item(client, String::from(&item), user).await;
    match result {
        Err(e) => CommandResult::Failure(e.to_string()),
        Ok(_) => CommandResult::Success(format!("Task with ID={} deleted successfully.", item))
    }
}