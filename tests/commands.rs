use std::sync::Mutex;

use clinv::database::init_db;
use rusqlite::Connection;
use clinv::commands;
use clinv::cli::Commands;

use once_cell::sync::Lazy;

static TEST_DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let connection = Connection::open_in_memory().unwrap();
    let _ = init_db(&connection);
    Mutex::new(connection)
});

#[test]
fn test_execute_command_list_clients() {
    let connection = TEST_DB.lock().unwrap();
    let result = commands::execute_command(&connection, Commands::ListClients);
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_client() {
    let connection = TEST_DB.lock().unwrap();

    // Add client
    connection.execute(
        "INSERT INTO client (name, nickname, email, phone_number) VALUES ('Bob', 'bobby', 'bob@example.com', '123')",
        [],
    ).unwrap();
    
    // Delete client
    let result = commands::execute_command(
        &connection,
        Commands::DeleteClient { client_nickname: Some("bobby".to_string()) },
    );
    assert!(result.is_ok());
    
    // Check client was deleted
    let mut stmt = connection.prepare("SELECT COUNT(*) FROM client").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_execute_command_list_invoices_empty() {
    let connection = TEST_DB.lock().unwrap();

    let result = commands::execute_command(&connection, Commands::ListInvoices { client_nickname: None });
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_invoice() {
    let connection = TEST_DB.lock().unwrap();

    // Insert a client and invoice
    connection.execute(
        "INSERT INTO client (name, nickname, email, phone_number) VALUES ('Carol', 'carr', 'carol@example.com', '555')",
        [],
    ).unwrap();

    let mut stmt = connection.prepare("SELECT id FROM client").unwrap();
    let id: Option<i32> = stmt.query_row([], |row| row.get(0)).unwrap();
    connection.execute(
        "INSERT INTO invoice (client_id, date) VALUES (?1, '2025-06-06')",
        [id.unwrap().to_string()],
    ).unwrap();
    // Insert an invoice item for completeness
    connection.execute(
        "INSERT INTO invoice_item (invoice_id, description, hours, rate, amount) VALUES (1, 'service', 2, 50, 100)",
        [],
    ).unwrap();
    
    // Delete the invoice
    let result = commands::execute_command(
        &connection,
        Commands::DeleteInvoice { invoice_id: Some("1".to_string()) },
    );
    assert!(result.is_ok());

    // Check invoice and invoice_item tables are now empty
    let mut stmt = connection.prepare("SELECT COUNT(*) FROM invoice").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
    let mut stmt = connection.prepare("SELECT COUNT(*) FROM invoice_item").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}
