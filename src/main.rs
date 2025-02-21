use std::sync::{Arc, Mutex};
mod types;
mod web_service;
mod serialisation;
mod command_handler;
mod console_service;
use crate::console_service::console_service::start_console;
use crate::types::{State};

#[tokio::main]
async fn main(){

    println!("Loading state");
    let state = if std::fs::exists("./state.json").unwrap() {
        println!("State file found");
        serialisation::perform_load()
    } else {
        println!("No state file found");
        State { items: Vec::new() }
    };

    let shared_state = Arc::new(Mutex::new(state));
    // Create a clone for the HTTP server
    let http_state = Arc::clone(&shared_state);

    println!("Starting WebService");
    let (close_tx, server_handle) = web_service::web_service::start_webservice(http_state).await;

    println!("Starting Console");
    start_console(shared_state).await.expect("Console failed to start");

    println!("Telling Server to shutdown");
    _ = close_tx.send(());

    println!("Gracefully shutting down");
    _ = server_handle.await;
}



