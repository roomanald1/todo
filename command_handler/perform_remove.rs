use crate::types;
use crate::command_handler::commands;
use crate::command_handler::commands::CommandResult;

pub(crate) fn perform_remove(state: &mut types::State, item: String) -> commands::CommandResult{
    let index = state.items.iter().position(|value| { value.id.to_string() == item }).unwrap();
    state.items.remove(index);
    CommandResult::Success
}