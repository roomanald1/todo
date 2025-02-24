use tokio::runtime::Runtime;
use tokio_postgres::Client;
use commands::CommandResult::Success;
use crate::command_handler::commands;
use crate::store::database;
use crate::types;
use commands::CommandResult::Failure;

pub(crate) fn perform_done_toggle(_: &mut types::State, item: String, done: bool, client: &Client) -> commands::CommandResult{

    let id = match item.parse::<i64>() {
        Ok(i) => i,
        Err(_) => return Failure("Invalid item ID".to_string()),
    };

    match done {
        true => Runtime::new().unwrap().block_on(database::mark_as_done(client, id)),
        false => Runtime::new().unwrap().block_on(database::mark_as_open(client, id)),
    };
        
    // let transformed = state.items.iter().map(|i| {
    //     let mut clone = i.clone();
    //     if i.id.to_string() == item {
    //         clone.completed = done;
    //         clone
    //     }else {
    //         clone
    //     }
    // }).collect::<Vec<types::TODO>>();
    // state.items = transformed;
    Success(format!("Task with ID={} updated successfully.",item))
}