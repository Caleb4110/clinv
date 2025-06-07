use crate::cli::Commands;
use crate::database;
use crate::utils;
use crate::utils::generate_pdf;
use chrono::prelude::*;
use rusqlite::{Connection, Result};

pub fn execute_command(connection: &Connection, command: Commands) -> Result<()> {
    // Execute the command
    match command {
        Commands::NewClient {
            name,
            email,
            phone_number,
        } => {
            println!("Creating new client...");
            // Prompt for name and email if not provided
            let name = name.unwrap_or_else(|| utils::prompt_for_str("Enter client name: "));
            let email = email.unwrap_or_else(|| utils::prompt_for_str("Enter client email: "));
            let phone_number = phone_number
                .unwrap_or_else(|| utils::prompt_for_str("Enter client phone number: "));

            database::new_client(connection, &name, &email, &phone_number)?;
            println!("Created client: {} <{}> <{}>", name, email, phone_number);

            Ok(())
        }
        Commands::NewInvoice { client_name } => {
            match client_name {
                Some(ref client_name) => {
                    println!("Creating invoice for client: {}...", client_name);
                }
                None => {
                    println!("Creating new invoice...");
                }
            }
            // Prompt for client name if not provided
            let client_name =
                client_name.unwrap_or_else(|| utils::prompt_for_str("Enter client name: "));
            let local: DateTime<Local> = Local::now();
            let date_string = local.format("%Y-%m-%d").to_string();
            let invoice_id = database::new_invoice(connection, &client_name, &date_string)?;

            println!(
                "Created invoice with id: {}, for client: {} ",
                invoice_id, client_name
            );

            // Get items
            let _items = utils::read_invoice_items(connection, invoice_id);
            println!("Items added to invoice with id: {}", invoice_id);

            Ok(())
        }
        Commands::ListClients => {
            println!("Listing all clients...");

            let clients = database::get_clients(connection)?;
            println!("===========");
            for client in clients {
                println!(
                    "id: {} \nname: {} \nemail: {}\nphone number: {}",
                    client.id, client.name, client.email, client.phone_number
                );
                println!("===========");
            }
            Ok(())
        }
        Commands::DeleteClient { client_name } => {
            match client_name {
                Some(ref client_name) => {
                    println!("Deleting client: {}...", client_name);
                }
                None => {
                    println!("Deleting client...");
                }
            }
            // Prompt for client ID if not provided
            let client_name =
                client_name.unwrap_or_else(|| utils::prompt_for_str("Enter client name: "));
            println!("Deleted client: {}", client_name);
            database::delete_client(connection, &client_name)?;
            Ok(())
        }
        Commands::ListInvoices { client_name } => {
            let invoices;
            match client_name {
                Some(ref client_name) => {
                    println!("Listing invoices for client: {}", client_name);
                    invoices = database::get_invoices(connection, Some(client_name))?;
                }
                None => {
                    println!("Listing all invoices...");
                    invoices = database::get_invoices(connection, None)?;
                }
            }

            println!("===========");
            for invoice in invoices {
                println!(
                    "id: {} \nclient id: {} \ndate: {}",
                    invoice.id, invoice.client_id, invoice.date
                );
                for item in invoice.items {
                    println!("\t++++++++");
                    println!(
                        "\titem id: {}\n\tdescription: {}\n\thours: {}\n\trate: {}\n\tamount: {}",
                        item.id, item.description, item.hours, item.rate, item.amount
                    );
                }
                println!("===========");
            }
            Ok(())
        }
        Commands::DeleteInvoice { invoice_id } => {
            match invoice_id {
                Some(ref invoice_id) => {
                    println!("Deleting invoice: {}...", invoice_id);
                }
                None => {
                    println!("Deleting invoice...");
                }
            }
            // Prompt for invoice ID if not provided
            let invoice_id =
                invoice_id.unwrap_or_else(|| utils::prompt_for_str("Enter invoice ID: "));
            database::delete_invoice(connection, &invoice_id)?;
            println!("Deleted invoice with id: {}", invoice_id);
            Ok(())
        }
        Commands::Generate { invoice_id } => {
            match invoice_id {
                Some(ref invoice_id) => {
                    println!("Generating invoice pdf for invoice: {}...", invoice_id);
                }
                None => {
                    println!("Generating invoice pdf...");
                }
            }
            let invoice_id =
                invoice_id.unwrap_or_else(|| utils::prompt_for_str("Enter invoice ID: "));
            let invoice = database::get_invoice(connection, &invoice_id)?;
            println!("===========");
            println!(
                "id: {}\nclient_name: {}\nclient_email: {}\ndate: {}",
                invoice.id, invoice.client_name, invoice.client_email, invoice.date
            );
            for item in &invoice.items {
                println!("\t++++++++");
                println!(
                    "\titem id: {}\n\tdescription: {}\n\thours: {}\n\trate: {}\n\tamount: {}",
                    item.id, item.description, item.hours, item.rate, item.amount
                );
            }
            println!("===========");

            let _ = generate_pdf(&invoice, "./template.html");

            Ok(())
        }
    }
}
