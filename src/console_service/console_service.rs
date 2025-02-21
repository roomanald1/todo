use std::io::stdin;
use std::sync::{Arc, Mutex};
use tokio::task::{spawn_blocking, JoinHandle};
use crate::command_handler::commands;
use crate::command_handler::commands::{CommandInput, CommandResult};
use crate::types::State;

fn handle_command(state: Arc<Mutex<State>>) {
    loop {
        println!("> ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut state = state.lock().unwrap();
        let should_continue = match commands::Command::execute(&mut state, CommandInput::CommandLine(input)) {
            CommandResult::Success(_) => true,
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

pub fn start_console(shared_state :Arc<Mutex<State>>) -> JoinHandle<()> {
    let handle = tokio::spawn({
        let shared_state = Arc::clone(&shared_state);
        async move {
            println!("Welcome to your TODO list");
            println!("Type 'h' for help");

            spawn_blocking(move || { handle_command(shared_state); }).await.unwrap();
        }
    });
    handle
}