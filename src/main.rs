use std::sync::Arc;
use tokio::sync::Mutex;
mod types;
mod web_service;
mod command_handler;
mod store;
use tracing::{info, span, Level};
use crate::store::database;
use crate::store::database::Database;
use crate::store::postgres_store::PostgresStore;
use crate::store::redis_store::RedisStore;

#[tokio::main]
async fn main() -> Result<(), String> {
    let file_appender = tracing_appender::rolling::hourly(".", "todo.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();

    let app_span = span!(Level::INFO, "Application");
    let _ = app_span.enter();

    info!("Starting Application");

    let mut db = Database::Redis(RedisStore::new());
    db.init().await?;

    //Uncomment this to sync redis and postgres
    //let mut backup = Database::Postgres(PostgresStore::new());
    //backup.init().await?;
    //database::sync(backup, db.clone()).await?;

    let result = web_service::web_service::start_webservice(Arc::new(Mutex::new(db))).await;
    result
}
