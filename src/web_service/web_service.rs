use std::sync::Arc;
use tokio::sync::Mutex;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::HeaderMap;
use axum::routing::{delete, get, put};
use tokio_postgres::Client;
use tower_http::cors::CorsLayer;
use crate::command_handler::commands::{Command, CommandInfo, CommandInput, CommandResult, HttpMethod};

async fn items_handler(
    command: CommandInfo<'_>,
    args: Vec<String>,
    client: Arc<Mutex<Client>>,
    headers: HeaderMap
    ) -> impl IntoResponse
{
    if !headers.contains_key("user")
    {
        return "user not specified".to_string()
    }

    let user = String::from(headers.get("user").unwrap().to_str().unwrap());
    match Command::execute( CommandInput::Http(command.keys[0].to_string(), args), Arc::clone(&client),user).await {
        CommandResult::Success(x) => x,
        CommandResult::Failure(x) => x,
        CommandResult::Exit => "Exiting".to_string()
    }
}
pub async fn start_webservice(client: Arc<Mutex<Client>>){
    let mut app = Router::new();
    for command in Command::command_info() {
        let client = Arc::clone(&client);
        match command.http_method {
            HttpMethod::Get => {
                let command = command.clone();
                app = app.route(command.http_path, get(|headers: HeaderMap| async move {
                    let db = Arc::clone(&client);
                    items_handler(command, Vec::new(),db, headers).await
                }));
            }
            HttpMethod::Delete => {
                let command = command.clone();
                app = app.route(command.http_path, delete(|Path(params): Path<Vec<(String,String)>>, headers: HeaderMap| async move {
                    let db = Arc::clone(&client);
                    items_handler(command, params.iter().map(|(_, value)| { String::from(value)}).collect(), db, headers).await
                }));
            },
            HttpMethod::Put => {
                let command = command.clone();
                app = app.route(command.http_path, put(|headers: HeaderMap,Json(value): Json<String>| async move{
                    let db = Arc::clone(&client);
                    items_handler(command, vec![value], db, headers).await
                }));
            }
            _ => {}
        }
    }

    app = app.layer(CorsLayer::permissive());

    let address = "0.0.0.0:3000";
    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to bind listener: {}", e),//TODO handle this
    };

    println!("Listening on {}", address);
    axum::serve(listener, app).await.expect("Failed to start server");

}