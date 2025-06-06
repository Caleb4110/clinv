use clinv::database;
use rusqlite::Connection;

fn setup_in_memory_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    // Run the same schema setup as your main database
    conn.execute(
        "CREATE TABLE client (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL
        )",
        [],
    ).unwrap();
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
    conn
}

#[test]
fn test_new_and_get_client() {
    let conn = setup_in_memory_db();
    database::new_client(&conn, "Alice", "alice@example.com", "12345").unwrap();
    let clients = database::get_clients(&conn).unwrap();
    assert_eq!(clients.len(), 1);
    assert_eq!(clients[0].name, "Alice");
    assert_eq!(clients[0].email, "alice@example.com");
    assert_eq!(clients[0].phone_number, "12345");
}

#[test]
fn test_delete_client() {
    let conn = setup_in_memory_db();
    database::new_client(&conn, "Bob", "bob@example.com", "67890").unwrap();
    let clients = database::get_clients(&conn).unwrap();
    let client_id = clients[0].id.to_string();
    database::delete_client(&conn, &client_id).unwrap();
    let clients = database::get_clients(&conn).unwrap();
    assert_eq!(clients.len(), 0);
}

#[test]
fn test_new_and_get_invoice() {
    let conn = setup_in_memory_db();
    // Add a client first
    database::new_client(&conn, "Carol", "carol@example.com", "55555").unwrap();
    let client_id = database::get_clients(&conn).unwrap()[0].id.to_string();
    let date = "2025-06-06";
    let invoice_id = database::new_invoice(&conn, &client_id, date).unwrap();
    assert_eq!(invoice_id, 1);
    // There should be a new invoice
    let invoices = database::get_invoices(&conn).unwrap();
    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].client_id.to_string(), client_id);
    assert_eq!(invoices[0].date, date);
    // Invoice should start with zero items
    assert_eq!(invoices[0].items.len(), 0);
}

#[test]
fn test_delete_invoice() {
    let conn = setup_in_memory_db();
    database::new_client(&conn, "Dave", "dave@example.com", "11111").unwrap();
    let client_id = database::get_clients(&conn).unwrap()[0].id.to_string();
    let date = "2025-06-06";
    let invoice_id = database::new_invoice(&conn, &client_id, date).unwrap();
    database::delete_invoice(&conn, &invoice_id.to_string()).unwrap();
    let invoices = database::get_invoices(&conn).unwrap();
    assert_eq!(invoices.len(), 0);
}
