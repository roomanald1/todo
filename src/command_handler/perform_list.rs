use comfy_table::Table;
use tokio::runtime::Runtime;
use tokio_postgres::Client;
use crate::store::database;
use crate::types;
use crate::command_handler::commands;
use crate::types::TODO;

pub(crate) fn perform_list_console(state: &types::State, open :Option<bool>, client: &Client) -> commands::CommandResult{
    let (_, table) = perform_list(state, open, client);
    commands::CommandResult::Success(table.to_string())
}


pub(crate) fn perform_list<'a>(_: &'a types::State, open :Option<bool>, client: &'a Client) -> (Vec< TODO>, Table) {
    let result = Runtime::new().unwrap().block_on(database::get_data(client, None));

    let items : Vec<TODO> = result.into_iter().filter(|i|{
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
        let id = match item.id {
            Some(i) => i.to_string(),
            None => "N/A".to_string(),
        };
        table.add_row(vec![id, item.description.clone(), item.added_on.clone(), item.completed.to_string()]);
    }
    (items, table)
}