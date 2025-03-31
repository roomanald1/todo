use crate::store::database::Database;
use crate::types::Todo;

pub(crate) async fn perform_get(database: &mut Database, user: String) -> Result<Vec<Todo>, String> {
    let items = database.get(user).await.map_err(|e| e.to_string())?;
    Ok(items)
}