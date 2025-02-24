use std::sync::{Arc, Mutex};
mod types;
mod web_service;
mod command_handler;
mod console_service;
mod store;

use crate::console_service::console_service::start_console;
use crate::types::State;

#[tokio::main]
async fn main(){

    println!("Starting Database");
    let (items, client) = store::database::init().await;
    let db = Arc::new(Mutex::new(client));
    let state = State { items};

    let shared_state = Arc::new(Mutex::new(state));
    let http_state = Arc::clone(&shared_state);
    let http_db = Arc::clone(&db);

    println!("Starting WebService");
    let (close_tx, server_handle) = web_service::web_service::start_webservice(http_state, http_db).await;

    println!("Starting Console");
    start_console(shared_state, db).await.expect("Console failed to start");

    println!("Telling Server to shutdown");
    _ = close_tx.send(());

    println!("Gracefully shutting down");
    _ = server_handle.await;
}
