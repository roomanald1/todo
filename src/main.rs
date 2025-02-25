use std::sync::Arc;
use tokio::sync::Mutex;
mod types;
mod web_service;
mod command_handler;
mod console_service;
mod store;

#[tokio::main]
async fn main(){

    println!("Starting Database");
    let client = store::database::init().await;
    let db: Arc<Mutex<tokio_postgres::Client>> = Arc::new(Mutex::new(client));
    let http_db = Arc::clone(&db);

    println!("Starting WebService");
    let (close_tx, server_handle) = web_service::web_service::start_webservice(http_db).await;

    let use_console = match std::env::var("USE_CONSOLE") {
        Ok(value) => value == "true",
        Err(_) => true,
    };
    
    if use_console {
        println!("Starting Console");
        console_service::console_service::start_console(db).await.expect("Console failed to start");

        println!("Telling Server to shutdown");
        _ = close_tx.send(());
    }

    println!("Waiting for server to finish");
    _ = server_handle.await;
}
