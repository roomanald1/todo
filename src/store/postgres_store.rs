use std::fs;
use std::sync::Arc;
use async_trait::async_trait;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::{Client, Error};
use tracing::info;
use crate::store::database::DatabaseBackend;
use crate::types::Todo;

#[derive(Clone)]
pub struct PostgresStore {
    client: Option<Arc<Client>>
}

#[async_trait]
impl DatabaseBackend for PostgresStore {
    async fn init(&mut self) -> Result<(), String> {
        info!("Connecting to Postgres DB");
        self.connect().await?;
        let client = self.client.as_ref().ok_or("Client not initialised".to_string())?;
        //let _ = drop_table(&client).await.map_err(|err| err.to_string());
        let _ = PostgresStore::create_table_if_not_exists(client).await.map_err(|e| {format!("Failed to create table {}", e)});
        Ok(())
    }

    async fn get(&mut self, user: String) -> Result<Vec<Todo>, String> {
        let client = self.client.as_ref().ok_or(format!("Client not initialised"))?;
        PostgresStore::get_data(client, user).await
    }

    async fn set(&mut self, user: String, items: Vec<Todo>) -> Result<(), String> {
        let client = self.client.as_ref().ok_or(format!("Client not initialised"))?;
        let _ = PostgresStore::upsert(client, user, items).await?;
        Ok(())
    }
}

impl PostgresStore {

    pub fn new() -> PostgresStore {
        PostgresStore { client: None}
    }

    async fn connect(&mut self) -> Result<(), String> {
        let db_url = "postgres://avnadmin:AVNS_GLDm0Kvw_n4n1jR9QVF@pg-321f6976-ronnie-9662.g.aivencloud.com:27821/defaultdb?sslmode=require";

        let cert = PostgresStore::load_certs()?;

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

        self.client = Some(Arc::new(client));
        Ok(())
    }


    fn load_certs() -> Result<Vec<u8>, String> {
        match fs::read("/usr/local/share/ca-certificates/ca.pem"){
            Ok(certs) => Ok(certs),
            Err(_) => match fs::read("./ca.pem"){
                Ok(certs) => Ok(certs),
                Err(error) => Err(format!("{}", error)),
            },
        }
    }

    async fn drop_table(client: &Client) -> Result<u64, Error> {
        // SQL command to drop the table
        let drop_table_query = "DROP TABLE IF EXISTS todo2";
        // Execute the query
        client.execute(drop_table_query, &[]).await
    }

    async fn create_table_if_not_exists(client: &Client) -> Result<u64, Error> {
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS todo2 (
            user_id TEXT PRIMARY KEY,
            items TEXT NOT NULL
        )",
                &[],
            )
            .await
    }

    async fn upsert(client: &Client, user_id: String, items: Vec<Todo>) -> Result<i64, String> {
        let row = client
            .execute("insert into todo2(user_id, items) values($1, $2)
             on conflict(user_id)
             do update set items = $2",
                     &[
                         &user_id,
                         &serde_json::to_string(&items).map_err(|e| e.to_string())?
                     ],
            )
            .await
            .map_err(|e| e.to_string())?;

        let id: i64 = i64::try_from(row).map_err(|e| e.to_string())?;
        Ok(id)
    }

    async fn get_data(client: &Client, user: String) -> Result<Vec<Todo>, String> {
        // Verify by selecting rows from the table
        let rows = client
            .query(
                "SELECT user_id, items
                      FROM todo2
                      WHERE user_id = $1",
                &[&user],
            )
            .await.map_err(|e| { format!("Failed to get data {}", e)})?;

        Ok(rows
            .iter()
            .filter_map(|row| {
                let items_str = row.get(1);
                let items: Option<Vec<Todo>> = match serde_json::from_str(items_str){
                    Ok(v) => Some(v),
                    Err(_) => None
                };
                items
            })
            .flat_map(|e| e)
            .collect())

    }
}
