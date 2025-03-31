use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{Json, Router};
use axum::http::HeaderMap;
use axum::routing::{get, put};
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};
use crate::command_handler::perform_list::{perform_get};
use crate::command_handler::perform_update::{perform_update};
use crate::store::database::Database;
use crate::types::Todo;

#[instrument]
fn get_user(headers: HeaderMap) -> Result<String, String>{
    let user = headers.get("user")
        .ok_or("user not specified")
        .map_err(|e| e.to_string())?
        .to_str()
        .map_err(|e| e.to_string())?;
    Ok(String::from(user))
}


pub async fn start_webservice(database: Arc<Mutex<Database>>) -> Result<(), String>{
    let app = Router::new()
        .route("/api/get", get({
            let db= Arc::clone(&database);
            move |headers:HeaderMap| async move {
                let user = get_user(headers)?;
                let mut connection = db.lock().await;
                let items = perform_get(&mut connection, user).await.map_err(|e|e.to_string())?;
                return Ok::<Json<Vec<Todo>>, String>(Json(items));
            }
         }))
        .route("/api/update", put({
            let db = Arc::clone(&database);
            move |headers:HeaderMap, Json(items)| async move {
                let user = get_user(headers)?;
                let mut connection = db.lock().await;
                let result = perform_update(&mut connection, user, items).await.map_err(|e|e.to_string())?;
                return Ok::<Json<Vec<Todo>>, String>(Json(result));
            }
        }))
        .layer(CorsLayer::permissive());

    let address = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(address).await.map_err(|e| {format!("Failed to bind to listener: {}", e)})?;

    info!("Listening on {}", address);
    axum::serve(listener, app.clone()).await.expect("Failed to start server");
    Ok({})
}