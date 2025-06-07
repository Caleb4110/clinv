use crate::models::{Client, Invoice, InvoiceForPdf, InvoiceItem};
use rusqlite::{Connection, OptionalExtension, Result};

pub fn connect() -> Result<Connection> {
    let connection = Connection::open("clinv.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS client (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL
        )",
        [],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS invoice (
            id INTEGER PRIMARY KEY,
            client_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            FOREIGN KEY (client_id) REFERENCES client(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS invoice_item (
            id INTEGER PRIMARY KEY,
            invoice_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            hours FLOAT NOT NULL,
            rate FLOAT NOT NULL,
            amount FLOAT NOT NULL,
            FOREIGN KEY (invoice_id) REFERENCES invoice(id)
        )",
        [],
    )?;

    Ok(connection)
}

pub fn new_client(
    connection: &Connection,
    name: &str,
    email: &str,
    phone_number: &str,
) -> Result<()> {
    connection.execute(
        "INSERT INTO client (name, email, phone_number) VALUES (?1, ?2, ?3)",
        &[name, email, phone_number],
    )?;
    Ok(())
}

pub fn delete_client(connection: &Connection, client_name: &str) -> Result<()> {
    connection.execute("DELETE FROM client WHERE name = ?1", &[client_name])?;
    Ok(())
}

pub fn get_clients(connection: &Connection) -> Result<Vec<Client>> {
    let mut statement = connection.prepare("SELECT * FROM client")?;

    let client_iter = statement.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone_number: row.get(3)?,
        })
    })?;

    let clients: Vec<Client> = client_iter.filter_map(Result::ok).collect();

    Ok(clients)
}

pub fn new_invoice(connection: &Connection, client_name: &str, date_string: &str) -> Result<i64> {
    // Check if client exists
    let client_exists: Option<i32> = connection
        .query_row(
            "SELECT id FROM client WHERE name = ?1",
            &[client_name],
            |row| row.get(0),
        )
        .optional()?;
    if client_exists.is_none() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    // Insert new invoice
    connection.execute(
        "INSERT INTO invoice (client_id, date) VALUES (?1, ?2)",
        &[&client_exists.unwrap().to_string(), date_string],
    )?;
    let invoice_id = connection.last_insert_rowid();

    Ok(invoice_id)
}

pub fn delete_invoice(connection: &Connection, invoice_id: &str) -> Result<()> {
    connection.execute(
        "DELETE FROM invoice_item WHERE invoice_id = ?1",
        &[invoice_id],
    )?;
    connection.execute("DELETE FROM invoice WHERE id = ?1", &[invoice_id])?;
    Ok(())
}

pub fn get_invoices(connection: &Connection, client_name: Option<&str>) -> Result<Vec<Invoice>> {
    let mut statement;
    let mut rows_iter;
    match client_name {
        Some(_) => {
            statement = connection.prepare(
                "SELECT
                invoice.id as invoice_id, invoice.client_id, invoice.date,
                invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
                invoice_item.rate, invoice_item.amount
            FROM invoice
            INNER JOIN client ON invoice.client_id = client.id
            LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
            WHERE client.name = ?1
            ORDER BY invoice.id",
            )?;
            rows_iter = statement.query([client_name])?;
        }
        None => {
            statement = connection.prepare(
                "SELECT
                invoice.id as invoice_id, invoice.client_id, invoice.date,
                invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
                invoice_item.rate, invoice_item.amount
            FROM invoice
            LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
            ORDER BY invoice.id",
            )?;
            rows_iter = statement.query([])?;
        }
    }

    let mut invoices = Vec::new();
    let mut current_invoice_id = None;
    let mut current_invoice = None;

    while let Some(row) = rows_iter.next()? {
        let invoice_id: i32 = row.get(0)?;
        if current_invoice_id != Some(invoice_id) {
            if let Some(invoice) = current_invoice.take() {
                invoices.push(invoice);
            }
            current_invoice_id = Some(invoice_id);
            current_invoice = Some(Invoice {
                id: invoice_id,
                client_id: row.get(1)?,
                date: row.get(2)?,
                items: Vec::new(),
            });
        }
        // If there's an item, add it
        if let Some(item_id) = row.get::<_, Option<i32>>(3)? {
            let item = InvoiceItem {
                id: item_id,
                description: row.get(4)?,
                hours: row.get(5)?,
                rate: row.get(6)?,
                amount: row.get(7)?,
            };
            if let Some(invoice) = current_invoice.as_mut() {
                invoice.items.push(item);
            }
        }
    }
    if let Some(invoice) = current_invoice {
        invoices.push(invoice);
    }

    Ok(invoices)
}

pub fn get_invoice(connection: &Connection, invoice_id: &str) -> Result<InvoiceForPdf> {
    let mut statement = connection.prepare(
        "SELECT 
            invoice.id as invoice_id, invoice.client_id, invoice.date,
            client.name, client.email, client.phone_number,
            invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
            invoice_item.rate, invoice_item.amount
        FROM invoice
        RIGHT JOIN client on invoice.client_id = client.id
        LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
        WHERE invoice.id = ?1
        ",
    )?;

    let mut rows_iter = statement.query([invoice_id])?;
    let mut items = Vec::new();
    let mut id = None;
    let mut client_name = None;
    let mut client_email = None;
    let mut client_phone_number = None;
    let mut date = None;

    while let Some(row) = rows_iter.next()? {
        if id.is_none() {
            id = Some(row.get(0)?);
            client_name = Some(row.get(3)?);
            client_email = Some(row.get(4)?);
            client_phone_number = Some(row.get(5)?);
            date = Some(row.get(2)?);
        }

        if let Some(item_id) = row.get::<_, Option<i32>>(6)? {
            items.push(InvoiceItem {
                id: item_id,
                description: row.get(7)?,
                hours: row.get(8)?,
                rate: row.get(9)?,
                amount: row.get(10)?,
            });
        }
    }

    if let (
        Some(id),
        Some(client_name),
        Some(client_email),
        Some(client_phone_number),
        Some(date),
    ) = (id, client_name, client_email, client_phone_number, date)
    {
        Ok(InvoiceForPdf {
            id,
            client_name,
            client_email,
            client_phone_number,
            date,
            items,
        })
    } else {
        // No invoice found
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
