mod cli;
mod commands;
mod database;
mod utils;
mod models;

use clap::Parser;
use cli::{map_command_words, Cli};

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
