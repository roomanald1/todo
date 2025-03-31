use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use redis::Connection;
use crate::types::Todo;
use redis::Commands;
use crate::store::database::DatabaseBackend;

#[derive(Clone)]
pub struct RedisStore{
    connection: Option<Arc<Mutex<Connection>>>
}

#[async_trait]
impl DatabaseBackend for RedisStore {
    async fn init(&mut self) -> Result<(), String>{
        let connection_str = "redis://ronnie:Prapr7fU_@redis-13230.c338.eu-west-2-1.ec2.redns.redis-cloud.com:13230";
        let client = redis::Client::open(connection_str).map_err(|e| e.to_string())?;
        let connection = client.get_connection().map_err(|e| e.to_string())?;
        self.connection = Some(Arc::new(Mutex::new(connection)));
        Ok(())
    }

    async fn get(&mut self, user: String) -> Result<Vec<Todo>, String> {
        match self.connection {
            Some(ref connection) => {
                let connection = connection.lock().map_err(|e| e.to_string());
                let result: Option<String> = connection?.get(user).map_err(|e| e.to_string())?;
                match result {
                    Some(value) => {
                        serde_json::from_str(&value).map_err(|e| e.to_string())
                    },
                    None => Ok(Vec::<Todo>::new())
                }
            },
            None => {
                Err(format!("No connection established"))
            }
        }

    }

    async fn set(&mut self, user: String, items: Vec<Todo>) -> Result<(), String> {
        match self.connection {
            Some(ref connection) => {
                let value = serde_json::to_string(&items).map_err(|e| e.to_string())?;
                let connection = connection.lock().map_err(|e| e.to_string());
                let _: () = connection?.set(user, value).map_err(|e| e.to_string())?;
                Ok(())
            },
            None => {
                Err(format!("No connection established"))
            }
        }

    }
}

impl RedisStore {

    pub fn new() -> RedisStore {
        RedisStore { connection: None}
    }
}

