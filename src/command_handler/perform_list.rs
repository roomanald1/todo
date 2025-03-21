use tokio_postgres::Client;
use tracing::instrument;
use crate::store::database;
use crate::types::Todo;


#[instrument]
pub(crate) async fn perform_get(client: &Client, user: String) -> Result<Vec<Todo>, String> {
    let items = database::get_data(client, user).await.map_err(|e| e.to_string())?;
    Ok(items)
}