use clinv::cli::Commands;
use clinv::commands;
use clinv::database::init_db;


fn setup() -> rusqlite::Connection {
    let connection = rusqlite::Connection::open_in_memory().unwrap();
    init_db(&connection).unwrap();
    connection
}

#[test]
fn test_execute_command_list_clients() {
    let connection = setup();
    let result = commands::execute_command(&connection, Commands::ListClients);
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_client() {
    let connection = setup();

    // Add client
    connection.execute(
        "INSERT INTO client (name, nickname, email, phone_number) VALUES ('Bob', 'bobby', 'bob@example.com', '123')",
        [],
    ).unwrap();

    // Delete client
    let result = commands::execute_command(
        &connection,
        Commands::DeleteClient {
            client_nickname: Some("bobby".to_string()),
        },
    );
    assert!(result.is_ok());

    // Check client was deleted
    let mut stmt = connection.prepare("SELECT COUNT(*) FROM client").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_execute_command_list_invoices_empty() {
    let connection = setup();

    let result = commands::execute_command(
        &connection,
        Commands::ListInvoices {
            client_nickname: None,
        },
    );
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_invoice() {
    let connection = setup();

    // Insert a client and invoice
    connection.execute(
        "INSERT INTO client (name, nickname, email, phone_number) VALUES ('Carol', 'carr', 'carol@example.com', '555')",
        [],
    ).unwrap();

    let client_id = connection.last_insert_rowid();

    connection
        .execute(
            "INSERT INTO invoice (client_id, date) VALUES (?1, '2025-06-06')",
            [client_id],
        )
        .unwrap();
    let invoice_id = connection.last_insert_rowid();

    // Insert an invoice item for completeness
    connection.execute(
        "INSERT INTO invoice_item (invoice_id, description, hours, rate, amount) VALUES (?1, 'service', 2, 50, 100)",
        [invoice_id],
    ).unwrap();

    // Delete the invoice
    let result = commands::execute_command(
        &connection,
        Commands::DeleteInvoice {
            invoice_id: Some(invoice_id.to_string()),
        },
    );
    assert!(result.is_ok());

    // Check invoice and invoice_item tables are now empty for this invoice_id
    let mut stmt = connection
        .prepare("SELECT COUNT(*) FROM invoice WHERE id = ?1")
        .unwrap();
    let count: i64 = stmt.query_row([invoice_id], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);

    let mut stmt = connection
        .prepare("SELECT COUNT(*) FROM invoice_item WHERE invoice_id = ?1")
        .unwrap();
    let count: i64 = stmt.query_row([invoice_id], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}
