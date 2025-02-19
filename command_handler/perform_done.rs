use commands::CommandResult::Success;
use crate::command_handler::commands;
use crate::types;

pub(crate) fn perform_done_toggle(state: &mut types::State, item: String, done: bool) -> commands::CommandResult{
    let transformed = state.items.iter().map(|i| {
        let mut clone = i.clone();
        if i.id.to_string() == item {
            clone.completed = done;
            clone
        }else {
            clone
        }
    }).collect::<Vec<types::TODO>>();
    state.items = transformed;
    Success
}

