use std::io::stdin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_postgres::Client;
use tracing::{instrument};
use crate::command_handler::commands;
use crate::command_handler::commands::{CommandInput, CommandResult};

#[instrument]
async fn handle_command(client: Arc<Mutex<Client>>) {
    loop {
        println!("\r\n> ");
        let mut input = String::new();
        match stdin().read_line(&mut input){
            Ok(_) => (),
            Err(_) => break,
        };

        let client = Arc::clone(&client);
        let should_continue = match commands::Command::execute(CommandInput::CommandLine(input), client, String::from("ronnie.day1@gmail.com")).await {
            CommandResult::Success(x) => {
                println!("{}", x);
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

pub fn start_console(client: Arc<Mutex<Client>>) -> JoinHandle<()> {
    tokio::spawn({
        async move {
            println!("Welcome to your TODO list");
            println!("Type 'h' for help");
            handle_command( client).await
        }
    })
}