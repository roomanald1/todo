use std::collections::{HashMap};
use crate::command_handler::perform_list::perform_get;
use crate::store::database::Database;
use crate::types::{Todo, TodoUpdate, UpdateAction};

async fn perform_set_internal(database: &mut Database, user: String, items :Vec<Todo>) -> Result<Vec<Todo>, String> {
    let _ = database.set(user, items.clone()).await.map_err(|e| e.to_string())?;
    Ok(items)
}


pub(crate) async fn perform_update(database: &mut Database, user: String, updates: Vec<TodoUpdate>) -> Result<Vec<Todo>, String> {
    let current = perform_get(database, user.clone()).await.map_err(|e| e.to_string())?;
    let mut current_by_id: HashMap<_, _> =   current.clone().into_iter().map(|data| (data.id, data)).collect();

    for update in updates.iter(){
        match update.action{
            UpdateAction::Update => {
                let item = current_by_id
                    .get_mut(&Some(update.id))
                    .ok_or("Cannot update missing item.".to_string())?;

                *item = update.clone().data.ok_or("Update missing data".to_string())?;
            }
            UpdateAction::Add => {
                current_by_id
                    .insert(Some(update.id), update.clone().data.ok_or("Update update data".to_string())?);
            },
            UpdateAction::Remove => {
                current_by_id
                    .remove(&Some(update.id))
                    .ok_or("unabled find item to remove".to_string())?;
            }
        }
    }
    perform_set_internal(database, user,  current_by_id.values().cloned().collect::<Vec<Todo>>()).await
}

