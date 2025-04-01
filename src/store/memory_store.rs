use std::collections::HashMap;
use async_trait::async_trait;
use crate::store::database::DatabaseBackend;
use crate::types::Todo;

#[derive(Debug, Clone)]
pub struct MemoryStore {
    map: HashMap<String, Vec<Todo>>,
}

impl MemoryStore {
    pub(crate) fn new() -> MemoryStore {
        MemoryStore { map: HashMap::new()  }
    }
}

#[async_trait]
impl DatabaseBackend for MemoryStore {
    async fn init(&mut self) -> Result<(), String> {
        self.map = HashMap::new();
        Ok(())
    }

    async fn get(&mut self, user: String) -> Result<Vec<Todo>, String> {
        self.map.get(&user).map(|e| e.clone()).ok_or(format!("{} not found", user))
    }

    async fn set(&mut self, user: String, items: Vec<Todo>) -> Result<(), String> {
        self.map.insert(user, items);
        Ok(())
    }
}