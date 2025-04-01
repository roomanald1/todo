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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::memory_store::MemoryStore;
    use crate::types::{Todo, TodoUpdate, UpdateAction};

    // Helper function to create a test database with MemoryStore
    async fn create_test_db() -> Database {
        let mut db = Database::Memory(MemoryStore::new());
        db.init().await.expect("Failed to initialize database");
        db
    }

    // Helper function to create a test Todo item
    fn create_todo(id: i64, text: &str, completed: bool) -> Todo {
        Todo {
            description: text.to_string(),
            id: Some(id),
            is_deleted: None,
            last_updated: None,
            user_id: "".to_string(),
            detail: None,
            completed,
            added_on: "".to_string(),
            due: None,
        }
    }

    // Helper function to create a TodoUpdate
    fn create_update(id: i64, action: UpdateAction, data: Option<Todo>) -> TodoUpdate {
        TodoUpdate {
            id,
            action,
            data,
        }
    }

    async fn setup_db_with_initial_data() -> (Database, String) {
        let mut db = create_test_db().await;
        let user = "test_user".to_string();
        let initial_todos = vec![
            create_todo(1, "Buy milk", false),
            create_todo(2, "Walk dog", true),
        ];
        perform_set_internal(&mut db, user.clone(), initial_todos)
            .await
            .expect("Setup failed");
        (db, user)
    }

    // =====================
    // Individual Tests
    // =====================

    #[tokio::test]
    async fn test_update_existing_item() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(
            1,
            UpdateAction::Update,
            Some(create_todo(1, "Buy almond milk", true)),
        )];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_ok());
        let todos = result.unwrap();
        let updated = todos.iter().find(|t| t.id == Some(1)).unwrap();
        assert_eq!(updated.description, "Buy almond milk");
        assert!(updated.completed);
    }

    #[tokio::test]
    async fn test_add_new_item() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(
            3,
            UpdateAction::Add,
            Some(create_todo(3, "New task", false)),
        )];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_ok());
        let todos = result.unwrap();
        assert_eq!(todos.len(), 3);
        assert!(todos.iter().any(|t| t.id == Some(3)));
    }

    #[tokio::test]
    async fn test_remove_item() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(2, UpdateAction::Remove, None)];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_ok());
        let todos = result.unwrap();
        assert_eq!(todos.len(), 1);
        assert!(!todos.iter().any(|t| t.id == Some(2)));
    }

    #[tokio::test]
    async fn test_update_nonexistent_item_fails() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(
            99,
            UpdateAction::Update,
            Some(create_todo(99, "Should fail", true)),
        )];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot update missing item.");
    }

    #[tokio::test]
    async fn test_remove_nonexistent_item_fails() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(99, UpdateAction::Remove, None)];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unabled find item to remove");
    }

    #[tokio::test]
    async fn test_add_with_missing_data_fails() {
        let (mut db, user) = setup_db_with_initial_data().await;

        let updates = vec![create_update(4, UpdateAction::Add, None)];

        let result = perform_update(&mut db, user, updates).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Update update data");
    }
}