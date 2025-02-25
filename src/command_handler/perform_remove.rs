use tokio_postgres::Client;
use crate::store::database;
use crate::command_handler::commands;
use crate::command_handler::commands::CommandResult;

pub(crate) async fn perform_remove(item: String, client: &Client) -> commands::CommandResult{

    let result = database::remove_item(client, String::from(&item)).await;
    match result {
        Err(e) => CommandResult::Failure(e.to_string()),
        Ok(_) => CommandResult::Success(format!("Task with ID={} deleted successfully.", item))
    }
}