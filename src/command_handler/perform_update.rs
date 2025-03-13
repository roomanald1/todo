use crate::store::database;
use tokio_postgres::Client;
use tracing::instrument;
use serde_json;

#[instrument]
pub(crate) async fn perform_update(client: &Client, user: String, all_items_str: String) -> Result<String, String>{
    let value = serde_json::from_str(&all_items_str).map_err(|e| e.to_string())?;
    let id = database::upsert(client, user, value).await.map_err(|e| e.to_string())?;
    Ok(format!("Successfully updated {}", id))
}
