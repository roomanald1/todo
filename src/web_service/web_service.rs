use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{delete, get, put};
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;
use tokio_postgres::Client;
use crate::command_handler::commands::{Command, CommandInfo, CommandInput, CommandResult, HttpMethod};
use crate::types::State;

async fn items_handler(
    shared_state: Arc<Mutex<State>>,
    command: CommandInfo<'_>,
    args: Vec<String>,
    client: Arc<Mutex<Client>>
    ) -> impl IntoResponse
{
    let shared_state = Arc::clone(&shared_state); // Move the state into the handler
    let mut state = shared_state.lock().unwrap();
    let connection = client.lock().unwrap();
    match Command::execute(&mut state, CommandInput::Http(command.keys[0].to_string(), args), &connection) {
        CommandResult::Success(x) => x,
        CommandResult::Failure(x) => x,
        CommandResult::Exit => "Exiting".to_string()
    }
}

pub async fn start_webservice(http_state: Arc<Mutex<State>>, client: Arc<Mutex<Client>>) -> (Sender<()>, JoinHandle<()>){
    let mut app = Router::new();
    for command in Command::command_info() {
        match command.http_method {
            HttpMethod::Get => {
                app = app.route(command.http_path, get(  {
                    let shared_state = Arc::clone(&http_state);
                    let db = Arc::clone(&client);
                    move || {
                        async move {
                            items_handler(shared_state, command, Vec::new(),db).await
                        }
                    }
                }));
            }
            HttpMethod::Delete => {
                app = app.route(command.http_path, delete({
                    let shared_state = Arc::clone(&http_state);
                    let db = Arc::clone(&client);
                    move |Path(params): Path<Vec<(String,String)>>| {
                        async move {
                            items_handler(shared_state, command, params.iter().map(|(_, value)| { String::from(value)}).collect(), db).await
                        }
                    }
                }));
            },
            HttpMethod::Put => {
                app = app.route(command.http_path, put({
                    let shared_state = Arc::clone(&http_state);
                    let db = Arc::clone(&client);
                    move |Json(value): Json<String>| {
                        async move { items_handler(shared_state, command, vec![value], db).await }
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
    (close_tx, server_handle)
}