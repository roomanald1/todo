use std::io::{stdin, stdout};
use std::sync::{Arc, Mutex};
use axum::{Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, put};
use crossterm::execute;
mod types;
mod state_serde;
mod command_handler;
use crossterm::terminal;
use tokio::task::spawn_blocking;
use command_handler::commands::CommandResult;
use crate::command_handler::commands;
use crate::command_handler::commands::{Command, CommandInfo, CommandInput, HttpMethod};
use crate::types::{State};

#[tokio::main]
async fn main(){

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let state = if std::fs::exists("./state.json").unwrap() {
        state_serde::perform_load()
    } else {
        State { items: Vec::new() }
    };

    let shared_state = Arc::new(Mutex::new(state));

    // Create a clone for the HTTP server
    let http_state = Arc::clone(&shared_state);


    println!("Starting WebService");

    let mut app = Router::new();

    for command in Command::command_info() {
        match command.http_method {
            HttpMethod::Get => {
                app = app.route(command.http_path, get({
                    let shared_state = Arc::clone(&http_state);
                    move || {
                        items_handler(shared_state, command, Vec::new())
                    }
                }));
            }
            HttpMethod::Delete => {
                app = app.route(command.http_path, delete({
                    let shared_state = Arc::clone(&http_state);
                    move |Path(params): Path<Vec<(String,String)>>| {
                        async move { items_handler(shared_state, command, params.iter().map(|(key, value)| { String::from(value)}).collect()).await }
                    }
                }));
            },
            HttpMethod::Put => {
                app = app.route(command.http_path, put({
                    let shared_state = Arc::clone(&http_state);
                    move |Json(value): Json<String>| {
                        async move { items_handler(shared_state, command, vec![value]).await }
                    }
                }));
            }
            _ => {}
        }
    }


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    let (close_tx, close_rx) = tokio::sync::oneshot::channel();

    let server_handle = tokio::spawn(async {
        axum::serve(listener, app).with_graceful_shutdown(async move {
            _ = close_rx.await
        }).await.unwrap();
    });

    let command_handle = tokio::spawn({
        let shared_state = Arc::clone(&shared_state);
        async move {
            println!("Welcome to your TODO list");
            println!("Type 'h' for help");

            spawn_blocking(move || {
                handle_command(shared_state);
            })
                .await
                .unwrap();
        }
    });

    command_handle.await.unwrap();

    println!("Telling Server to shutdown");
    _ = close_tx.send(());

    println!("Gracefully shutting down");
    _ = server_handle.await;


}


async fn items_handler(
    shared_state: Arc<Mutex<State>>,
    command: CommandInfo<'_>,
    args: Vec<String>) -> impl IntoResponse
{
    let shared_state = Arc::clone(&shared_state); // Move the state into the handler
    let mut state = shared_state.lock().unwrap();
    match Command::execute(&mut state, CommandInput::Http(command.keys[0].to_string(), args)){
        CommandResult::Success(x) => x,
        CommandResult::Failure(x) => x,
        CommandResult::Exit => "Exiting".to_string()
    }
}

fn handle_command(state: Arc<Mutex<State>>) {
    loop {
        println!("> ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut state = state.lock().unwrap();
        let should_continue = match commands::Command::execute(&mut state, CommandInput::CommandLine(input)) {
            CommandResult::Success(_) => true,
            CommandResult::Failure(x) => {
                println!("Error: {}", x);
                true
            }
            CommandResult::Exit => false,
        };

        if !should_continue {
            break;
        }
    }
}

