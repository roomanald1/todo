use comfy_table::Table;
use tokio_postgres::Client;
use tracing::instrument;
use crate::store::database;
use crate::types::Todo;

#[instrument]
pub(crate) async fn perform_get(client: &Client, user: String) -> Result<String, String> {
    let items = database::get_data(client, user).await.map_err(|e| e.to_string())?;
    let as_str = serde_json::to_string(&items).map_err(|e| e.to_string())?;
    Ok(as_str)
}


#[instrument]
pub fn as_table(items: Vec<Todo>) -> Table{
    let mut table = Table::new();
    table
        .set_header(vec!["ID", "Description", "Added On", "Completed", "Due", "Detail"]);

    for item in &items {
        let id = match item.id {
            Some(i) => i.to_string(),
            None => "N/A".to_string(),
        };
        table.add_row(vec![
            id,
            item.description.clone(),
            item.added_on.clone(),
            item.completed.to_string(),
            item.due.clone().unwrap_or_else(|| "".to_string()),
            item.detail.clone().unwrap_or_else(|| "".to_string()),
        ]);
    }
    table
}