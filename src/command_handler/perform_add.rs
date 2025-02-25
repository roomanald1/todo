use crate::store::database;
use crate::types;
use crate::command_handler::commands::CommandResult;
use chrono::Utc;
use tokio::runtime::Runtime;
use tokio_postgres::Client;
use crate::command_handler::commands;

pub(crate) fn perform_add(description: String, client: &Client) -> commands::CommandResult {

    let item = types::Todo {
        id: None,
        user_id: String::from("user_123"),
        description,
        added_on: Utc::now().to_string(),
        completed: false
    };

    match Runtime::new(){ 
        Ok(r) => {
            let id = r.block_on(database::upsert_item(client, item));
            match id {
                Ok(id) => CommandResult::Success(format!("Task with ID={} added successfully.",id)),
                Err(e) => CommandResult::Failure(e.to_string())
            }
        },
        Err(e) => CommandResult::Failure(e.to_string())
    }   
}
