use std::sync::Arc;
use tokio_postgres::Client;

use crate::command_handler::perform_add::perform_add;
use crate::command_handler::perform_list::{perform_list, perform_list_console};
use crate::command_handler::perform_remove::perform_remove;
use crate::types;
use crate::command_handler::commands;
use crate::command_handler::perform_done::perform_done_toggle;

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

#[derive(Clone)]
pub struct CommandInfo<'a> {
    pub keys: Vec<&'a str>,
    pub description: &'a str,
    pub http_path: &'a str,
    pub http_method: HttpMethod,
    pub handler: Arc<dyn Fn(&CommandInput, &mut types::State, String, &Client)-> CommandResult + Send + Sync>
}

// Manual Debug implementation for CommandInfo, skipping the `handler` field.
impl<'a> std::fmt::Debug for CommandInfo<'a> {
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

    pub fn get_command(command_key: &String) -> Option<CommandInfo> {
        Self::command_info()
            .into_iter()
            .find(|x| x.keys.contains(&command_key.to_lowercase().as_str()))
    }

    pub fn execute(state: &mut types::State, input: CommandInput, client: &Client) -> CommandResult {

        let (key, args) = match input {
            CommandInput::CommandLine(ref x) => {
                let input_split = x.trim().split(" ").collect::<Vec<&str>>();
                let command_item = input_split[0].to_string();
                let command_args = input_split.into_iter().skip(1).collect::<Vec<&str>>().join(" ").clone();
                (command_item, command_args)
            },
            CommandInput::Http(ref x, ref y) => (String::from(x), y.join(" "))
        };


        let x = match Command::get_command(&key) {
            Some(c) => {
                (c.handler)(&input, state, args, client)
            },
            None => {
                CommandResult::Failure(format!("Invalid Command {}", key))
            }
        };
        x
    }

    pub fn command_info() -> Vec<CommandInfo<'static>> {
        vec![
            CommandInfo {
                keys: vec!["a", "add"],
                description: "Add a new item",
                http_method: HttpMethod::Put,
                http_path: "/api/items/add",
                handler: Arc::new(|_, state, value, client| perform_add(state, value, client))
            },
            CommandInfo {
                keys: vec!["l", "list", "ls"],
                description: "List all items",
                http_path: "/api/items",
                http_method: HttpMethod::Get,
                handler: Arc::new(|input, state, _, client| {
                    match input {
                        CommandInput::CommandLine(_) => {
                            perform_list_console(state, None, client)
                        },
                        CommandInput::Http(_, _) => {
                            let (result, _) = perform_list(state, None, client);
                            CommandResult::Success(serde_json::to_string(&result).unwrap())
                        }
                    }

                })
            },
            CommandInfo {
                keys: vec!["l:open", "list:open", "ls:open"],
                description: "List open items",
                http_path: "/api/items/open",
                http_method: HttpMethod::Get,
                handler: Arc::new(|input, state, _, client| {
                    match input {
                        CommandInput::CommandLine(_) => {
                            perform_list_console(state, Some(true), client)
                        },
                        CommandInput::Http(_, _) => {
                            let (result, _) = perform_list(state, Some(true), client);
                            CommandResult::Success(serde_json::to_string(&result).unwrap())
                        }
                    }

                })
            },
            CommandInfo {
                keys: vec!["l:done", "list:done", "ls:done"],
                description: "List done items",
                http_method: HttpMethod::Get,
                http_path: "/api/items/done",
                handler: Arc::new(|input, state, _, client| {
                    match input {
                        CommandInput::CommandLine(_) => {
                            perform_list_console(state, Some(false), client)
                        },
                        CommandInput::Http(_, _) => {
                            let (result, _) = perform_list(state, Some(false), client);
                            CommandResult::Success(serde_json::to_string(&result).unwrap())
                        }
                    }

                })
            },
            CommandInfo {
                keys: vec!["r", "remove"],
                description: "Remove an item",
                http_path: "/api/items/remove/{id}",
                http_method: HttpMethod::Delete,
                handler: Arc::new(|_, state, value, client| perform_remove(state, value, client))
            },
            CommandInfo {
                keys: vec!["x", "exit", "q", "quit"],
                description: "Exit the application",
                http_path: "none",
                http_method: HttpMethod::None,
                handler: Arc::new(|_, _, _, _| CommandResult::Exit)
            },
            CommandInfo {
                keys: vec!["h", "help"],
                description: "Help!",
                http_path: "none",
                http_method: HttpMethod::None,
                handler: Arc::new(|_,_, _, _| {
                    println!("Available Commands:");
                    for x in commands::Command::command_info() {
                        println!("-> \t[{}]\t\t\t{}", &x.keys.join(","),  String::from(x.description));
                    }
                    CommandResult::Success(String::from("Available Commands:"))
                })
            },
            CommandInfo {
                keys: vec!["done", "d"],
                description: "Mark as done",
                http_path: "/api/items/{id}/done",
                http_method: HttpMethod::Put,
                handler: Arc::new(|_, state, value, client| {
                    perform_done_toggle(state, value, true, client)
                })
            },
            CommandInfo {
                keys: vec!["open", "o"],
                description: "Mark as Open",
                http_path: "/api/items/{id}/open",
                http_method: HttpMethod::Put,
                handler: Arc::new(|_, state, value, client| {
                    perform_done_toggle(state, value, false, client)
                })
            }
        ]
    }

}

