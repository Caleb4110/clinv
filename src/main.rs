mod cli;
mod commands;
mod database;
mod utils;

use clap::Parser;
use cli::{map_command_words, Cli};

#[derive(Debug)]
struct Client {
    id: i32,
    name: String,
    email: String,
    phone_number: String
}

#[derive(Debug)]
struct InvoiceItem {
    id: i32,
    description: String,
    hours: f64,
    rate: f64,
    amount: f64,
}

#[derive(Debug)]
struct Invoice {
    id: i32,
    client_id: i32,
    date: String,
    items: Vec<InvoiceItem>
}

struct InvoiceForPdf {
    id: i32,
    client_name: String,
    client_email: String,
    client_phone_number: String,
    date: String,
    items: Vec<InvoiceItem>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let connection = database::connect()?;

    // Determine the command to execute
    let command = match cli.command {
        Some(cmd) => cmd,
        None => match map_command_words(&cli.raw_command.words) {
            Some(cmd) => {
                cmd
            }
            None => {
                return Err(format!("Unknown command: {}", cli.raw_command.words.join(" ")).into());
            }
        },
    };

    commands::execute_command(&connection, command)?;
    Ok(())
}
