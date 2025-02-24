use crate::store::database;
use crate::types;
use crate::command_handler::commands::CommandResult;
use chrono::Utc;
use tokio::runtime::Runtime;
use tokio_postgres::Client;
use crate::command_handler::commands;

pub(crate) fn perform_add(_: &mut types::State, description: String, client: &Client) -> commands::CommandResult {

    let item = types::TODO {
        id: None,
        user_id: String::from("user_123"),
        description: description,
        added_on: Utc::now().to_string(),
        completed: false
    };

    let id = Runtime::new().unwrap().block_on(database::upsert_item(client, item));
    // let id =  state.items.iter().map(|x| x.id.clone()).max().unwrap_or(0) + 1;
    // state.items.push(
    // );
    CommandResult::Success(format!("Task with ID={} added successfully.",id.to_string()))
}
