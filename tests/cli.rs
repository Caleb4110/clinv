use clinv::cli::{map_command_words, Cli, Commands};
use clap::Parser;

#[test]
fn test_map_command_words_new_client() {
    let words = vec!["new".to_string(), "client".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::NewClient { name, email, phone_number }) => {
            assert!(name.is_none());
            assert!(email.is_none());
            assert!(phone_number.is_none());
        }
        _ => panic!("Expected Some(Commands::NewClient)"),
    }
}

#[test]
fn test_map_command_words_new_invoice() {
    let words = vec!["new".to_string(), "invoice".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::NewInvoice { client }) => assert!(client.is_none()),
        _ => panic!("Expected Some(Commands::NewInvoice)"),
    }
}

#[test]
fn test_map_command_words_list_clients() {
    let words = vec!["list".to_string(), "clients".to_string()];
    let cmd = map_command_words(&words);
    assert!(matches!(cmd, Some(Commands::ListClients)));
}

#[test]
fn test_map_command_words_list_invoices() {
    let words = vec!["list".to_string(), "invoices".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::ListInvoices { client }) => assert!(client.is_none()),
        _ => panic!("Expected Some(Commands::ListInvoices)"),
    }
}

#[test]
fn test_map_command_words_delete_client() {
    let words = vec!["delete".to_string(), "client".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::DeleteClient { client_id }) => assert!(client_id.is_none()),
        _ => panic!("Expected Some(Commands::DeleteClient)"),
    }
}

#[test]
fn test_map_command_words_delete_invoice() {
    let words = vec!["delete".to_string(), "invoice".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::DeleteInvoice { invoice_id }) => assert!(invoice_id.is_none()),
        _ => panic!("Expected Some(Commands::DeleteInvoice)"),
    }
}

#[test]
fn test_map_command_words_generate() {
    let words = vec!["generate".to_string()];
    let cmd = map_command_words(&words);
    match cmd {
        Some(Commands::Generate { invoice_id }) => assert!(invoice_id.is_none()),
        _ => panic!("Expected Some(Commands::Generate)"),
    }
}

#[test]
fn test_map_command_words_none() {
    let words: Vec<String> = vec![];
    let cmd = map_command_words(&words);
    assert!(cmd.is_none());
}

#[test]
fn test_cli_parse_new_client() {
    // Simulate: clinv new client --name Alice --email alice@example.com --phone-number 1234
    let cli = Cli::parse_from([
        "clinv",
        "new-client",
        "--name", "Alice",
        "--email", "alice@example.com",
        "--phone-number", "1234",
    ]);
    match cli.command {
        Some(Commands::NewClient { name, email, phone_number }) => {
            assert_eq!(name.as_deref(), Some("Alice"));
            assert_eq!(email.as_deref(), Some("alice@example.com"));
            assert_eq!(phone_number.as_deref(), Some("1234"));
        }
        _ => panic!("Expected NewClient command"),
    }
}

#[test]
fn test_cli_parse_list_clients() {
    let cli = Cli::parse_from(["clinv", "list-clients"]);
    match cli.command {
        Some(Commands::ListClients) => {}
        _ => panic!("Expected ListClients"),
    }
}

#[test]
fn test_raw_command_input_collection() {
    // Simulate passing raw command: clinv foo bar baz
    let cli = Cli::parse_from(["clinv", "foo", "bar", "baz"]);
    // If no subcommand matched, raw_command.words will contain ["foo", "bar", "baz"]
    assert_eq!(cli.raw_command.words, vec!["foo", "bar", "baz"]);
}
