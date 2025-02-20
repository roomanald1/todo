use crate::types;
use crate::command_handler::commands::CommandResult;
use chrono::Utc;
use crate::command_handler::commands;

pub(crate) fn perform_add(state: &mut types::State, item: String) -> commands::CommandResult {
    let id =  (state.items.len() as u32) + 1;
    state.items.push(types::TODO {
        id,
        description: item,
        added_on: Utc::now().to_string(),
        completed: false}
    );
    println!("Added");
    CommandResult::Success(format!("Task with ID={} added successfully.",id.to_string()))
}
