use comfy_table::Table;
use tokio_postgres::Client;
use crate::store::database;
use crate::command_handler::commands;
use crate::types::Todo;

pub(crate) async fn perform_list_console(open :Option<bool>, client: &Client) -> commands::CommandResult{
    match perform_list(open, client).await {
        Ok(data) => commands::CommandResult::Success(as_table(data).to_string()),
        Err(e) => commands::CommandResult::Failure(e.to_string()) 
    }
}

pub(crate) async fn perform_list_http(open :Option<bool>, client: &Client) -> commands::CommandResult{
    match perform_list(open, client).await{
        Ok(result) =>  match serde_json::to_string(&result)
        {
            Ok(x) => commands::CommandResult::Success(x),
            Err(_) => commands::CommandResult::Failure(String::from("Failed to serialize response"))
        },
        Err(err) => commands::CommandResult::Failure(err.to_string())
    }
}

pub(crate) async fn perform_list(open :Option<bool>, client: &Client) -> Result<Vec< Todo>, String> {
    match database::get_data(client, None).await {
            Ok(data) => Ok(data.into_iter().filter(|i| {
                match open {
                    Some(true) => !i.completed,
                    Some(false) => i.completed,
                    None => true,
                }
            }).collect()),
            Err(e) => Err(e.to_string())
        }
}


pub fn as_table(items: Vec<Todo>) -> Table{
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
    table
}