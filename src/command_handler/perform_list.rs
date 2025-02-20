use comfy_table::Table;
use crate::types;
use crate::command_handler::commands;
use crate::types::TODO;

pub(crate) fn perform_list_console(state: &types::State, open :Option<bool>) -> commands::CommandResult{
    let (_, table) = perform_list(state, open);
    println!("{}", table);
    commands::CommandResult::Success(table.to_string())
}


pub(crate) fn perform_list(state: &types::State, open :Option<bool>) -> (Vec<&TODO>, Table) {
    let items : Vec<&TODO> =state.items.iter().filter(|i|{
        match open {
            Some(true) => !i.completed,
            Some(false) => i.completed,
            None => true,
        }
    }).collect();

    let mut table = Table::new();
    table
        .set_header(vec!["ID", "Description", "Added On", "Completed"]);

    for item in &items {
        table.add_row(vec![item.id.clone().to_string(), item.description.clone(), item.added_on.clone(), item.completed.to_string()]);
    }
    (items, table)
}