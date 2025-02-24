use std::io::stdin;
use std::sync::{Arc, Mutex};
use tokio::task::{spawn_blocking, JoinHandle};
use tokio_postgres::Client;
use crate::command_handler::commands;
use crate::command_handler::commands::{CommandInput, CommandResult};
use crate::types::State;

fn handle_command(state: Arc<Mutex<State>>, client: Arc<Mutex<Client>>) {
    loop {
        println!("\r\n> ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let connection = client.lock().unwrap();

        let mut state = state.lock().unwrap();
        let should_continue = match commands::Command::execute(&mut state, CommandInput::CommandLine(input), &connection) {
            CommandResult::Success(x) => {
                print!("{}", x);
                true
            },
            CommandResult::Failure(x) => {
                println!("Error: {}", x);
                true
            }
            CommandResult::Exit => false,
        };

        if !should_continue {
            break;
        }
    }
}

pub fn start_console(shared_state :Arc<Mutex<State>>, client: Arc<Mutex<Client>>) -> JoinHandle<()> {
    let handle = tokio::spawn({
        let shared_state = Arc::clone(&shared_state);
        let db = Arc::clone(&client);
        async move {
            println!("Welcome to your TODO list");
            println!("Type 'h' for help");

            spawn_blocking(move || {
                handle_command(shared_state, db);
            }).await.unwrap();
        }
    });
    handle
}