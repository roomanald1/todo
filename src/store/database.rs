use async_trait::async_trait;
use crate::store::memory_store::MemoryStore;
use crate::store::postgres_store::PostgresStore;
use crate::store::redis_store::RedisStore;
use crate::types::Todo;

#[derive(Clone)]
pub enum Database {
    Postgres(PostgresStore),
    Redis(RedisStore),
    Memory(MemoryStore),
}

#[async_trait]
pub trait DatabaseBackend {
    async fn init(&mut self) -> Result<(), String>;
    async fn get(&mut self, user: String) -> Result<Vec<Todo>, String>;
    async fn set(&mut self, user: String, items: Vec<Todo>) -> Result<(), String>;
}

impl Database {
    pub async fn init(&mut self) -> Result<(), String> {
        match self{
            Database::Postgres(postgres) => {
                postgres.init().await
            }
            Database::Redis(redis) => {
                redis.init().await
            },
            Database::Memory(memory) => {
                memory.init().await
            }
        }
    }

    pub async fn get(&mut self, user: String) -> Result<Vec<Todo>, String> {
        match self{
            Database::Postgres(postgres) => {
                postgres.get(user).await
            }
            Database::Redis(redis) => {
                redis.get(user).await
            },
            Database::Memory(memory) => {
                memory.get(user).await
            }
        }
    }

    pub async fn set(&mut self, user:String, items: Vec<Todo>) -> Result<(), String>{
        match self{
            Database::Postgres(postgres) => {
                postgres.set(user, items).await
            }
            Database::Redis(redis) => {
                redis.set(user, items).await
            },
            Database::Memory(memory) => {
                memory.set(user, items).await
            }
        }
    }
}