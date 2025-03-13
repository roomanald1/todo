use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use crate::command_handler::perform_list::{perform_get};
use futures::{future::BoxFuture, FutureExt};
use tracing::{instrument};
use crate::command_handler::perform_update::{perform_update};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Add(String),
    List,
    Remove,
    MarkAsDone,
    MarkAsUndone,
    Save,
    Exit
}

type CommandHandler = dyn Fn(Arc<CommandInput>, String, Arc<Mutex<Client>>, String) -> BoxFuture<'static, CommandResult> + Send + Sync;

#[derive(Clone)]
pub struct CommandInfo<'a> {
    pub keys: Vec<&'a str>,
    pub description: &'a str,
    pub http_path: &'a str,
    pub http_method: HttpMethod,
    pub handler: Arc<CommandHandler>
}

// Manual Debug implementation for CommandInfo, skipping the `handler` field.
impl std::fmt::Debug for CommandInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandInfo")
            .field("keys", &self.keys)
            .field("description", &self.description)
            .field("http_path", &self.http_path)
            .field("http_method", &self.http_method)
            .finish()
    }
}


#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    None
}



#[derive(Debug, PartialEq)]
pub enum CommandResult{
    Success(String),
    Failure(String),
    Exit
}

#[derive(Debug)]
pub enum CommandInput {
    CommandLine(String),
    Http(String, Vec<String>)
}

impl Command {

    #[instrument]
    pub fn get_command(command_key: &str) -> Option<CommandInfo> {
        Self::command_info()
            .into_iter()
            .find(|x| x.keys.contains(&command_key.to_lowercase().as_str()))
    }

    #[instrument]
    pub async fn execute(input: CommandInput, client: Arc<Mutex<Client>>, user: String) -> CommandResult {

        let (key, args) = match input {
            CommandInput::CommandLine(ref x) => {
                let input_split = x.trim().split(" ").collect::<Vec<&str>>();
                let command_item = input_split[0].to_string();
                let command_args = input_split.into_iter().skip(1).collect::<Vec<&str>>().join(" ").clone();
                (command_item, command_args)
            },
            CommandInput::Http(ref x, ref y) => (String::from(x), y.join(" "))
        };

        match Command::get_command(&key) {
            Some(c) => {
                (c.handler)(input.into(), args, client, user).await
            },
            None => {
                CommandResult::Failure(format!("Invalid Command {}", key))
            }
        }
    }

    #[instrument]
    pub fn command_info() -> Vec<CommandInfo<'static>> {
        vec![
            CommandInfo {
                keys: vec!["get"],
                description: "Get all items for a user",
                http_path: "/api/get",
                http_method: HttpMethod::Get,
                handler: Arc::new(move |_input, _, client, user| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match perform_get(&connection, user).await {
                            Ok(v) => CommandResult::Success(v),
                            Err(e) => CommandResult::Failure(e)
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["set"],
                description: "Set all items for a user",
                http_method: HttpMethod::Put,
                http_path: "/api/set",
                handler: Arc::new(move |_, value, client, user| {
                    let client = Arc::clone(&client);
                    async move{
                        let connection = client.lock().await;
                        match perform_update(&connection, user, value).await {
                            Ok(v) => CommandResult::Success(v),
                            Err(e) => CommandResult::Failure(e)
                        }
                    }.boxed()
                })
            },
        ]
        }

}

