use crate::store::database::Database;
use crate::types::Todo;

pub(crate) async fn perform_get(database: &mut Database, user: String) -> Result<Vec<Todo>, String> {
    let items = database.get(user).await.map_err(|e| e.to_string())?;
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::memory_store::MemoryStore;
    use crate::types::{Todo};

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



    #[tokio::test]
    async fn test_nonexistant_user() {
        let mut db = create_test_db().await;
        let user = "test_user".to_string();

        let error_result = perform_get(&mut db, user.clone()).await;
        assert!(error_result.is_err());
        assert_eq!(error_result.unwrap_err(), "test_user not found");
    }

    #[tokio::test]
    async fn test_initial_data() {
        let mut db = create_test_db().await;
        db.set("test_user".to_string(),
               vec![
                   create_todo(1, "Buy milk", false),
                   create_todo(2, "Walk dog", true),
        ]).await.unwrap();

        let user = "test_user".to_string();
        let result = perform_get(&mut db, user.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}