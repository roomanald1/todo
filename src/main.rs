use std::sync::Arc;
use tokio::sync::Mutex;
mod types;
mod web_service;
mod command_handler;
mod console_service;
mod store;

#[tokio::main]
async fn main(){
    let mode = std::env::var("MODE").unwrap_or("web".to_string());
    let db = store::database::init().await;
    match mode.as_ref() {
        "console" => console_service::console_service::start_console(Arc::new(Mutex::new(db))).await.expect("Console failed to start"),
        "web" => web_service::web_service::start_webservice(Arc::new(Mutex::new(db))).await,
        mode  => println!("Invalid mode {}", mode)
    }
}
