use clinv::database::{self, init_db};
use rusqlite::Connection;

fn setup() -> Connection {
    let connection = Connection::open_in_memory().unwrap();
    init_db(&connection).unwrap();
    connection
}

#[test]
fn test_new_and_get_client() {
    let connection = setup();

    database::new_client(&connection, "Alice", "al", "alice@example.com", "12345").unwrap();
    let clients = database::get_clients(&connection).unwrap();
    assert_eq!(clients.len(), 1);
    assert_eq!(clients[0].name, "Alice");
    assert_eq!(clients[0].nickname, "al");
    assert_eq!(clients[0].email, "alice@example.com");
    assert_eq!(clients[0].phone_number, "12345");
}

#[test]
fn test_delete_client() {
    let connection = setup();

    database::new_client(&connection, "Bob", "bobby", "bob@example.com", "67890").unwrap();
    let clients_size = database::get_clients(&connection).unwrap().len();
    let client_nickname = "bobby".to_string();
    database::delete_client(&connection, &client_nickname).unwrap();
    let clients_new_size = database::get_clients(&connection).unwrap().len();
    assert_eq!(clients_size - 1, clients_new_size);
}

#[test]
fn test_new_and_get_invoice() {
    let connection = setup();

    // Add a client first
    database::new_client(&connection, "Carol", "car", "carol@example.com", "55555").unwrap();
    let client_id = database::get_clients(&connection).unwrap()[0].id.to_string();
    let client_nickname = database::get_clients(&connection).unwrap()[0].nickname.to_string();
    let date = "2025-06-06";
    let invoice_id = database::new_invoice(&connection, &client_nickname, date).unwrap();
    assert_eq!(invoice_id, 1);

    // There should be a new invoice
    let invoices = database::get_invoices(&connection, Some(&client_nickname)).unwrap();
    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].client_id.to_string(), client_id);
    assert_eq!(invoices[0].date, date);
    // Invoice should start with zero items
    assert_eq!(invoices[0].items.len(), 0);
}

#[test]
fn test_delete_invoice() {
    let connection = setup();

    database::new_client(&connection, "Dave", "davey", "dave@example.com", "11111").unwrap();
    let client_nickname = database::get_clients(&connection).unwrap()[0].nickname.to_string();
    let date = "2025-06-06";
    let invoice_id = database::new_invoice(&connection, &client_nickname, date).unwrap();
    database::delete_invoice(&connection, &invoice_id.to_string()).unwrap();
    let invoices = database::get_invoices(&connection, Some(&client_nickname)).unwrap();
    assert_eq!(invoices.len(), 0);
}
