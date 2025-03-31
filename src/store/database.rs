use crate::store::postgres_store::PostgresStore;
use crate::store::redis_store::RedisStore;
use crate::types::Todo;

#[derive(Clone)]
pub enum Database {
    Postgres(PostgresStore),
    Redis(RedisStore)
}

impl Database {
    pub async fn init(&mut self) -> Result<(), String> {
        match self{
            Database::Postgres(postgres) => {
                postgres.init().await
            }
            Database::Redis(redis) => {
                redis.init().await
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
            }
        }
    }
}