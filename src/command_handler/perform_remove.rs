use tokio::runtime::Runtime;
use tokio_postgres::Client;
use crate::store::database;
use crate::types;
use crate::command_handler::commands;
use crate::command_handler::commands::CommandResult;

pub(crate) fn perform_remove(_: &mut types::State, item: String, client: &Client) -> commands::CommandResult{
    let result = Runtime::new().unwrap().block_on(database::remove_item(client, String::from(&item)));
    match result {
        Err(e) => CommandResult::Failure(e.to_string()),
        Ok(_) => CommandResult::Success(format!("Task with ID={} deleted successfully.", item))
    }

    // let index = state.items.iter().position(|value| { value.id.to_string() == item }).unwrap();
    // state.items.remove(index);
    // CommandResult::Success(format!("Task with ID={} deleted successfully.",item))
}