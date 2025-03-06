use crate::types::Todo;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use tokio_postgres::{Client, Error};
use tracing::{info, instrument};

#[instrument]
async fn connect() -> Result<Client, String> {
    let db_url = "postgres://avnadmin:AVNS_GLDm0Kvw_n4n1jR9QVF@pg-321f6976-ronnie-9662.g.aivencloud.com:27821/defaultdb?sslmode=require";

    let cert = fs::read("./src/ca.pem")
        .map_err(|e| { format!("Failed to read CA certificate: {}", e) })?;

    let cert = Certificate::from_pem(&cert)
        .map_err(|e| { format!("Failed to parse CA certificate: {}", e) })?;

    // Build the TLS connector with the custom certificate
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| { format!("Failed to build TLS connector: {}", e) })?;

    let connector = MakeTlsConnector::new(connector);

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(db_url, connector)
        .await
        .map_err(|e| { format!("Failed to connect to the database: {}", e) })?;

    // Spawn a task to manage the connection to the database
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });

    Ok(client)
}

#[instrument]
async fn drop_table(client: &Client) -> Result<u64, Error> {
    // SQL command to drop the table
    let drop_table_query = "DROP TABLE IF EXISTS todo";
    // Execute the query
    client.execute(drop_table_query, &[]).await
}

#[instrument]
async fn create_table_if_not_exists(client: &Client) -> Result<u64, Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS todo (
            id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
            user_id TEXT NOT NULL,
            description TEXT NOT NULL,
            added_on TEXT NOT NULL,
            completed BOOLEAN NOT NULL,
            detail TEXT,
            due TEXT
        )",
            &[],
        )
        .await
}

#[instrument]
pub async fn remove_item(client: &Client, item: String, user: String) -> Result<u64, tokio_postgres::Error> {
    let command = "DELETE FROM todo WHERE id = $1 AND user_id = $2";
    client.execute(command, &[&item.parse::<i64>().unwrap(),&user]).await
}

#[instrument]
pub async fn upsert_item(client: &Client, item: Todo) -> Result<i64, String> {
    let row = if let Some(id) = item.id {
        // If id is provided, override the generated value
        client
            .execute(
                "UPDATE todo
                    SET completed = $2, description = $4, due = $5, detail = $6
                    WHERE id = $1
                    AND user_id = $3;",
                &[
                    &id,
                    &item.completed,
                    &item.user_id,
                    &item.description,
                    &item.due,
                    &item.detail,
                ],
            )
            .await
            .map_err(|e| e.to_string())?
    } else {
        // If id is None, let PostgreSQL generate it
        client
            .execute(
                "INSERT INTO todo (user_id, description, added_on, completed, due, detail)
                 VALUES ($1, $2, $3, $4, $5, $6)
                 RETURNING id",
                &[
                    &item.user_id,
                    &item.description,
                    &item.added_on,
                    &item.completed,
                    &item.due,
                    &item.detail,
                ],
            )
            .await
            .map_err(|e| e.to_string())?
    };

    let id: i64 = i64::try_from(row).map_err(|e| e.to_string())?;
    Ok(id)
}

#[instrument]
pub async fn mark_item(client: &Client, id: i64, done: bool, user: String) -> Result<u64, Error> {
    client
    .execute(
        "UPDATE todo
        SET completed = $2
        WHERE id = $1
        AND user_id = $3;",
        &[&id, &done, &user],
    )
    .await
}

#[instrument]
pub async fn init() -> Result<Client, String> {
    info!("Connecting to Postgres DB");
    let client = connect().await?;
    //let _ = drop_table(&client).await.map_err(|err| err.to_string());
    let _ = create_table_if_not_exists(&client).await.map_err(|e| {format!("Failed to create table {}", e)});
    Ok(client)
}

#[instrument]
pub async fn get_data(client: &Client, _: Option<bool>, user: String) -> Result<Vec<Todo>, String> {
    // Verify by selecting rows from the table
    let rows = client
        .query(
            "SELECT id, user_id, description, added_on, completed, due, detail
                      FROM todo
                      WHERE user_id = $1
                      ORDER BY id ASC",
            &[&user],
        )
        .await.map_err(|e| { format!("Failed to get data {}", e)})?;

    Ok(rows
        .iter()
        .map(|row| {
            Todo {
                id: row.get(0),
                user_id : row.get(1),
                description: row.get(2),
                added_on: row.get(3),
                completed: row.get(4),
                due: row.get(5),
                detail: row.get(6)
            }
        })
        .collect())

}
