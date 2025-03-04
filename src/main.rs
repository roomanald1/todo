use std::sync::Arc;
use tokio::sync::Mutex;
mod types;
mod web_service;
mod command_handler;
mod console_service;
mod store;
use tracing_subscriber;
use tracing::{info, span, Level};
use tracing_appender;

#[tokio::main]
async fn main() -> Result<(), String>{

    let file_appender = tracing_appender::rolling::hourly(".", "todo.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();

    let app_span = span!(Level::INFO, "Application");
    let _ = app_span.enter();

    info!("Starting Application");
    let mode = std::env::var("MODE").unwrap_or("web".to_string());
    info!("Mode ={}", mode);

    let db = store::database::init().await?;

    match mode.as_ref() {
        "console" => Ok(console_service::console_service::start_console(Arc::new(Mutex::new(db))).await.expect("Console failed to start")),
        "web" => Ok(web_service::web_service::start_webservice(Arc::new(Mutex::new(db))).await?),
        mode  => Err(format!("Invalid mode {}", mode))
    }
}
