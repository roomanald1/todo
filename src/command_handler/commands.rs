use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use crate::command_handler::perform_add::perform_add;
use crate::command_handler::perform_list::{perform_list_console, perform_list_http};
use crate::command_handler::perform_remove::perform_remove;
use crate::command_handler::commands;
use crate::command_handler::perform_done::perform_done_toggle;
use crate::store::database::mark_item;
use futures::{future::BoxFuture, FutureExt};

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

type CommandHandler = dyn Fn(Arc<CommandInput>, String, Arc<Mutex<Client>>) -> BoxFuture<'static, CommandResult> + Send + Sync;

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
pub enum CommandInput {
    CommandLine(String),
    Http(String, Vec<String>)
}

impl Command {

    pub fn get_command(command_key: &str) -> Option<CommandInfo> {
        Self::command_info()
            .into_iter()
            .find(|x| x.keys.contains(&command_key.to_lowercase().as_str()))
    }

    pub async fn execute(input: CommandInput, client: Arc<Mutex<Client>>) -> CommandResult {

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
                (c.handler)(input.into(), args, client).await
            },
            None => {
                CommandResult::Failure(format!("Invalid Command {}", key))
            }
        }
    }

    pub fn command_info() -> Vec<CommandInfo<'static>> {
        vec![
            CommandInfo {
                keys: vec!["a", "add"],
                description: "Add a new item",
                http_method: HttpMethod::Put,
                http_path: "/api/items/add",
                handler: Arc::new(move |_, value, client| {
                    let client = Arc::clone(&client);
                    async move{
                        let connection = client.lock().await;
                        perform_add(value, &connection)
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["l", "list", "ls"],
                description: "List all items",
                http_path: "/api/items",
                http_method: HttpMethod::Get,
                handler: Arc::new(move |input, _, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match &*input {
                            CommandInput::CommandLine(_) => perform_list_console(None, &connection).await,
                            CommandInput::Http(_, _) => perform_list_http(None, &connection).await
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["l:open", "list:open", "ls:open"],
                description: "List open items",
                http_path: "/api/items/open",
                http_method: HttpMethod::Get,
                handler: Arc::new(move |input, _, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match &*input {
                            CommandInput::CommandLine(_) => perform_list_console(Some(true), &connection).await,
                            CommandInput::Http(_, _) => perform_list_http(Some(true), &connection).await
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["l:done", "list:done", "ls:done"],
                description: "List done items",
                http_method: HttpMethod::Get,
                http_path: "/api/items/done",
                handler: Arc::new(move |input, _, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match &*input {
                            CommandInput::CommandLine(_) => perform_list_console(Some(false), &connection).await,
                            CommandInput::Http(_, _) => perform_list_http(Some(false), &connection).await
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["r", "remove"],
                description: "Remove an item",
                http_path: "/api/items/remove/{id}",
                http_method: HttpMethod::Delete,
                handler: Arc::new(move |_, value, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        perform_remove(value, &connection).await
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["x", "exit", "q", "quit"],
                description: "Exit the application",
                http_path: "none",
                http_method: HttpMethod::None,
                handler: Arc::new(|_, _, _| async move {CommandResult::Exit}.boxed())
            },
            CommandInfo {
                keys: vec!["h", "help"],
                description: "Help!",
                http_path: "none",
                http_method: HttpMethod::None,
                handler: Arc::new(|_, _, _| {
                    async move {
                        println!("Available Commands:");
                        for x in commands::Command::command_info() {
                            println!("-> \t[{}]\t\t\t{}", &x.keys.join(","),  String::from(x.description));
                        }
                        CommandResult::Success(String::from("Available Commands:"))
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["done", "d"],
                description: "Mark as done",
                http_path: "/api/items/{id}/done",
                http_method: HttpMethod::Put,
                handler: Arc::new(move |_, value, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match value.parse::<i64>() {
                            Ok(id) => match mark_item(&connection, id, true).await{
                                    Ok(_) => CommandResult::Success(format!("Task with ID={} updated successfully.",id)),
                                    Err(_) => CommandResult::Failure("Invalid item ID".to_string()),
                                },
                                Err(_) => CommandResult::Failure("Invalid item ID".to_string()),
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["undone", "u"],
                description: "Mark as undone",
                http_path: "/api/items/{id}/undone",
                http_method: HttpMethod::Put,
                handler: Arc::new(move |_, value, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        match value.parse::<i64>() {
                            Ok(id) => match mark_item(&connection, id, false).await{
                                    Ok(_) => CommandResult::Success(format!("Task with ID={} updated successfully.",id)),
                                    Err(_) => CommandResult::Failure("Invalid item ID".to_string()),
                                },
                                Err(_) => CommandResult::Failure("Invalid item ID".to_string()),
                        }
                    }.boxed()
                })
            },
            CommandInfo {
                keys: vec!["open", "o"],
                description: "Mark as Open",
                http_path: "/api/items/{id}/open",
                http_method: HttpMethod::Put,
                handler: Arc::new(move |_, value, client| {
                    let client = Arc::clone(&client);
                    async move {
                        let connection = client.lock().await;
                        perform_done_toggle(value, false, &connection).await
                    }.boxed()
                })
            }
        ]
        }

}

