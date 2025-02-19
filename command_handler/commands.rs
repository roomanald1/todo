use crate::command_handler::perform_add::perform_add;
use crate::command_handler::perform_list::{ perform_list_console};
use crate::command_handler::perform_remove::perform_remove;
use crate::state_serde::perform_save;
use crate::types;
use crate::command_handler::commands;
use crate::command_handler::perform_done::{ perform_done_toggle};

#[derive(Debug, Clone)]
pub enum Command {
    Add(String),
    List,
    Remove,
    MarkAsDone,
    MarkAsUndone,
    Save,
    Exit
}

pub struct CommandInfo<'a> {
    pub keys: Vec<&'a str>,
    pub description: &'a str,
    pub handler: Box<dyn Fn(&mut types::State, String)-> CommandResult>
}


pub enum CommandResult{
    Success,
    Failure(String),
    Exit
}

impl Command {
    pub fn execute(state: &mut types::State, input: &String) -> CommandResult{
        let input_split = input.trim().split(" ").collect::<Vec<&str>>();
        let command_item = input_split[0];
        let command_args = &input_split.into_iter().skip(1).collect::<Vec<&str>>().join(" ").clone();

        let binding = Self::command_info();

        let command = binding
            .iter()
            .filter(|x| {
                x.keys.contains(&command_item.to_lowercase().as_str())
            })
            .collect::<Vec<&CommandInfo>>();

        match command.first() {
            Some(c) => {
               (c.handler)(state, String::from(command_args))
            },
            None => {
                CommandResult::Failure(String::from("Invalid Command") + command_item)
            }
        }
    }

    // Map command descriptions to keys
    pub fn command_info() -> Vec<CommandInfo<'static>> {
        vec![
            CommandInfo {
                keys: vec!["a", "add"],
                description: "Add a new item",
                handler: Box::new(|state, value| perform_add(state, value))
            },
            CommandInfo {
                keys: vec!["l", "list", "ls"],
                description: "List all items",
                handler: Box::new(|state, _| perform_list_console(state, None))
            },
            CommandInfo {
                keys: vec!["l:open", "list:open", "ls:open"],
                description: "List open items",
                handler: Box::new(|state, _| perform_list_console(state, Some(true)))
            },
            CommandInfo {
                keys: vec!["l:done", "list:done", "ls:done"],
                description: "List open items",
                handler: Box::new(|state, _| perform_list_console(state, Some(false)))
            },
            CommandInfo {
                keys: vec!["r", "remove"],
                description: "Remove an item",
                handler: Box::new(|state, value| perform_remove(state, value))
            },
            CommandInfo {
                keys: vec!["x", "exit", "q", "quit"],
                description: "Exit the application",
                handler: Box::new(|_, _| CommandResult::Exit)
            },
            CommandInfo {
                keys: vec!["s", "save"],
                description: "Save state",
                handler: Box::new(|state, _| {
                    perform_save(state);
                    CommandResult::Success
                })
            },
            CommandInfo {
                keys: vec!["h", "help"],
                description: "Help!",
                handler: Box::new(|_, _| {
                    println!("Available Commands:");
                    for x in commands::Command::command_info() {
                        println!("-> \t[{}]\t\t\t{}", &x.keys.join(","),  String::from(x.description));
                    }
                    CommandResult::Success
                })
            },
            CommandInfo {
                keys: vec!["done", "d"],
                description: "Mark as done",
                handler: Box::new(|state, value| {
                    perform_done_toggle(state, value, true);
                    CommandResult::Success
                })
            },
            CommandInfo {
                keys: vec!["open", "o"],
                description: "Mark as Open",
                handler: Box::new(|state, value| {
                    perform_done_toggle(state, value, false);
                    CommandResult::Success
                })
            }
        ]
    }

}

