use std::collections::{HashMap};
use crate::store::database;
use tokio_postgres::Client;
use tracing::instrument;
use crate::command_handler::perform_list::perform_get;
use crate::types::{Todo, TodoUpdate, UpdateAction};

async fn perform_set_internal(client: &Client, user: String, items :Vec<Todo>) -> Result<Vec<Todo>, String> {
    let _ = database::upsert(client, user, items.clone()).await.map_err(|e| e.to_string())?;
    Ok(items)
}

#[instrument]
pub(crate) async fn perform_update(client: &Client, user: String, updates: Vec<TodoUpdate>) -> Result<Vec<Todo>, String> {
    let current = perform_get(client, user.clone()).await.map_err(|e| e.to_string())?;
    let mut current_by_id: HashMap<_, _> =   current.clone().into_iter().map(|data| (data.id, data)).collect();

    for update in updates.iter(){
        match update.action{
            UpdateAction::Update => {
                let item = current_by_id
                    .get_mut(&Some(update.id))
                    .ok_or(format!("Cannot update missing item."))?;

                *item = update.clone().data.ok_or(format!("Update missing data"))?;
            }
            UpdateAction::Add => {
                current_by_id
                    .insert(Some(update.id), update.clone().data.ok_or(format!("Update update data"))?);
            },
            UpdateAction::Remove => {
                current_by_id
                    .remove(&Some(update.id))
                    .ok_or(format!("unabled find item to remove"))?;
            }
        }
    }
    perform_set_internal(client, user,  current_by_id.values().cloned().collect::<Vec<Todo>>()).await
}

