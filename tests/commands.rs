use rusqlite::Connection;
use clinv::commands;
use clinv::cli::Commands;

// Helper to setup an in-memory client table
fn setup_in_memory_client_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE client (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL
        )",
        [],
    ).unwrap();
}
fn setup_in_memory_invoice_tables(conn: &Connection) {
    setup_in_memory_client_table(conn);
    conn.execute(
        "CREATE TABLE invoice (
            id INTEGER PRIMARY KEY,
            client_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            FOREIGN KEY (client_id) REFERENCES client(id)
        )",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE invoice_item (
            id INTEGER PRIMARY KEY,
            invoice_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            hours FLOAT NOT NULL,
            rate FLOAT NOT NULL,
            amount FLOAT NOT NULL,
            FOREIGN KEY (invoice_id) REFERENCES invoice(id)
        )",
        [],
    ).unwrap();
}

#[test]
fn test_execute_command_list_clients() {
    let conn = Connection::open_in_memory().unwrap();
    setup_in_memory_client_table(&conn);

    let result = commands::execute_command(&conn, Commands::ListClients);
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_client_with_id() {
    let conn = Connection::open_in_memory().unwrap();
    setup_in_memory_client_table(&conn);

    conn.execute(
        "INSERT INTO client (name, email, phone_number) VALUES ('Bob', 'bob@example.com', '123')",
        [],
    ).unwrap();
    // Client id will be 1 for the first insert
    let result = commands::execute_command(
        &conn,
        Commands::DeleteClient { client_id: Some("1".to_string()) },
    );
    assert!(result.is_ok());

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM client").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_execute_command_list_invoices_empty() {
    let conn = Connection::open_in_memory().unwrap();
    setup_in_memory_invoice_tables(&conn);

    let result = commands::execute_command(&conn, Commands::ListInvoices { client: None });
    assert!(result.is_ok());
}

#[test]
fn test_execute_command_delete_invoice_with_id() {
    let conn = Connection::open_in_memory().unwrap();
    setup_in_memory_invoice_tables(&conn);

    // Insert a client and invoice
    conn.execute(
        "INSERT INTO client (name, email, phone_number) VALUES ('Carol', 'carol@example.com', '555')",
        [],
    ).unwrap();
    conn.execute(
        "INSERT INTO invoice (client_id, date) VALUES (1, '2025-06-06')",
        [],
    ).unwrap();
    // Insert an invoice item for completeness
    conn.execute(
        "INSERT INTO invoice_item (invoice_id, description, hours, rate, amount) VALUES (1, 'service', 2, 50, 100)",
        [],
    ).unwrap();

    let result = commands::execute_command(
        &conn,
        Commands::DeleteInvoice { invoice_id: Some("1".to_string()) },
    );
    assert!(result.is_ok());

    // Check invoice and invoice_item tables are now empty
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM invoice").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM invoice_item").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}
