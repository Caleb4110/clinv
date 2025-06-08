use clap::Parser;
use clinv::cli::{map_command_words, Cli};
use clinv::{commands, database};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let connection = Connection::open("./clinv.db")?;
    database::init_db(&connection)?;

    // Determine the command to execute
    let command = match cli.command {
        Some(cmd) => cmd,
        None => match map_command_words(&cli.raw_command.words) {
            Some(cmd) => cmd,
            None => {
                return Err(format!("Unknown command: {}", cli.raw_command.words.join(" ")).into());
            }
        },
    };

    commands::execute_command(&connection, command)?;
    Ok(())
}
