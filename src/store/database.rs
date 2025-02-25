use crate::types::Todo;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use tokio_postgres::{Client, Error};

async fn connect() -> Client {
    let db_url = "postgres://avnadmin:AVNS_GLDm0Kvw_n4n1jR9QVF@pg-321f6976-ronnie-9662.g.aivencloud.com:27821/defaultdb?sslmode=require";

    let cert = fs::read("./src/ca.pem")
        .map_err(|e| {
            eprintln!("Failed to read CA certificate: {}", e);
            e
        })
        .expect("Failed to read CA certificate");//TODO 
    let cert = Certificate::from_pem(&cert)
        .map_err(|e| {
            eprintln!("Failed to parse CA certificate: {}", e);
            e
        })
        .expect("Failed to parse CA certificate");//TODO 

    // Build the TLS connector with the custom certificate
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| {
            eprintln!("Failed to build TLS connector: {}", e);
            e
        })
        .expect("Failed to build TLS connector");

    let connector = MakeTlsConnector::new(connector);

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(db_url, connector)
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to the database: {}", e);
            e
        })
        .expect("Failed to connect to the database");

    // Spawn a task to manage the connection to the database
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });

    client
}

async fn drop_table(client: &Client) -> Result<u64, Error> {
    // SQL command to drop the table
    let drop_table_query = "DROP TABLE IF EXISTS todo";
    // Execute the query
    client.execute(drop_table_query, &[]).await
}

async fn create_table_if_not_exists(client: &Client) -> Result<u64, Error> {
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
}

pub async fn remove_item(client: &Client, item: String) -> Result<u64, tokio_postgres::Error> {
    let command = format!("DELETE FROM todo WHERE id = {}", item);
    client.execute(&command, &[]).await
}

pub async fn upsert_item(client: &Client, item: Todo) -> Result<u64, String> {
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
        .await;

    match row {
        Ok(row) => {
            match u64::try_from(row.get::<_, i64>(0)) {
                Ok(id) => Ok(id),
                Err(_) => Err("Failed to convert row ID to u64".to_string()),
            }
        },
        Err(e) => Err(e.to_string()),
    }
}


pub async fn mark_item(client: &Client, id: i64, done: bool) -> Result<u64, Error> {
    client
    .execute(
        "UPDATE todo
        SET completed = $2
        WHERE id = $1;",
        &[&id, &done],
    )
    .await
}

pub async fn init() -> Client {
    let client = connect().await;
    match create_table_if_not_exists(&client).await {
        Ok(_) => (),    
        Err(e) => panic!("Failed to create table: {}", e)
    }
    client
}

pub async fn get_data(client: &Client, _: Option<bool>) -> Result<Vec<Todo>, String> {
    // Verify by selecting rows from the table
    let result = client
        .query(
            "SELECT id, user_id, description, added_on, completed FROM todo",
            &[],
        )
        .await;

    match result {
        Ok(rows) => {
            Ok(rows
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
        
                    Todo {
                        id,
                        user_id,
                        description,
                        added_on,
                        completed,
                    }
                })
                .collect())
        }, 
        Err(e) => Err(format!("Failed to fetch data from the database. {}", e))
    }
   
       
}
