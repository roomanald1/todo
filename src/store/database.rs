use crate::types::{State, TODO};
use chrono::Utc;
use comfy_table::Table;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

async fn connect() -> Client {
    let db_url = "postgres://avnadmin:AVNS_GLDm0Kvw_n4n1jR9QVF@pg-321f6976-ronnie-9662.g.aivencloud.com:27821/defaultdb?sslmode=require";

    let cert = fs::read("./src/ca.pem")
        .map_err(|e| {
            eprintln!("Failed to read CA certificate: {}", e);
            e
        })
        .unwrap();
    let cert = Certificate::from_pem(&cert)
        .map_err(|e| {
            eprintln!("Failed to parse CA certificate: {}", e);
            e
        })
        .unwrap();

    // Build the TLS connector with the custom certificate
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| {
            eprintln!("Failed to build TLS connector: {}", e);
            e
        })
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(db_url, connector)
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to the database: {}", e);
            e
        })
        .unwrap();

    // Spawn a task to manage the connection to the database
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });

    client
}

async fn drop_table(client: &Client) {
    // SQL command to drop the table
    let drop_table_query = "DROP TABLE IF EXISTS todo";

    // Execute the query
    client.execute(drop_table_query, &[]).await.unwrap();

    println!("Table dropped successfully!");
}

async fn create_table_if_not_exists(client: &Client) {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS todo (
            id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
            user_id TEXT NOT NULL,
            description TEXT NOT NULL,
            added_on TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
            &[],
        )
        .await
        .unwrap();
}

pub async fn remove_item(client: &Client, item: String) -> Result<u64, tokio_postgres::Error> {
    let command = format!("DELETE FROM todo WHERE id = {}", item);
    client.execute(&command, &[]).await
}

pub async fn upsert_item(client: &Client, item: TODO) -> u64 {
    let row = client
        .query_one(
            "INSERT INTO todo (user_id, description, added_on, completed)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET description = EXCLUDED.description,
                added_on = EXCLUDED.added_on,
                completed = EXCLUDED.completed,
                user_id = EXCLUDED.user_id
            RETURNING id",
            &[
                &item.user_id,
                &item.description,
                &item.added_on,
                &item.completed,
            ],
        )
        .await
        .unwrap();
    let id = match u64::try_from(row.get::<_, i64>(0)) {
        Ok(id) => id,
        Err(_) => 0,
    };
    println!("Item {} inserted successfully!", id);
    id
}

pub async fn mark_as_done(client: &Client, id: i64) {
    client
        .execute(
            "UPDATE todo
                SET completed = true
                WHERE id = $1;",
            &[&id],
        )
        .await
        .unwrap();
    }

pub async fn mark_as_open(client: &Client, id: i64) {
    client
        .execute(
            "UPDATE todo
            SET completed = false
            WHERE id = $1;",
            &[&id],
        )
        .await
        .unwrap();
}

pub async fn init() -> (Vec<TODO>, Client) {
    let client = connect().await;
    //drop_table(&client).await;
    create_table_if_not_exists(&client).await;
    let data = get_data(&client, None).await;
    (data, client)
}

pub async fn get_data(client: &Client, _: Option<bool>) -> Vec<TODO> {
    // Verify by selecting rows from the table
    client
        .query(
            "SELECT id, user_id, description, added_on, completed FROM todo",
            &[],
        )
        .await
        .unwrap()
        .iter()
        .map(|row| {
            let raw_id: i64 = row.get(0);
            let id = match u64::try_from(raw_id) {
                Ok(id) => Some(id),
                Err(_) => None,
            };
            let user_id: String = row.get(1);
            let description: String = row.get(2);
            let added_on: String = row.get(3);
            let completed: bool = row.get(4);

            return TODO {
                id,
                user_id,
                description,
                added_on,
                completed,
            };
        })
        .collect()
}

pub async fn test(_: Arc<Mutex<State>>) {
    let client = connect().await;

    drop_table(&client).await;

    create_table_if_not_exists(&client).await;

    upsert_item(
        &client,
        TODO {
            id: None,
            user_id: String::from("user_123"),
            description: String::from("item"),
            added_on: Utc::now().to_string(),
            completed: false,
        },
    )
    .await;

    let result = get_data(&client, Some(true)).await;
    let mut table = Table::new();
    table.set_header(vec!["ID", "Description", "Added On", "Completed"]);

    for item in &result {
        let id = match item.id {
            Some(i) => i.to_string(),
            None => "N/A".to_string(),
        };
        table.add_row(vec![
            id,
            item.description.clone(),
            item.added_on.clone(),
            item.completed.to_string(),
        ]);
    }
    println!("{}", table);

    println!("Test finished");
}
