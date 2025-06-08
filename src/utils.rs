use crate::models::InvoiceForPdf;
use chrono::{Duration, NaiveDate};
use rusqlite::{params, Connection};
use std::path::Path;
use std::error::Error;
use std::{
    fs,
    io::{self, Write},
};
use wkhtmltopdf::{self, Orientation, PdfApplication};

// Generic prompt function
pub fn prompt(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn prompt_for_f64(prompt_msg: &str) -> f64 {
    loop {
        let input = prompt(prompt_msg);
        if input.is_empty() {
            println!("Value must not be empty");
            continue;
        }
        match input.parse::<f64>() {
            Ok(n) => return n,
            Err(e) => println!("Not a valid number: {}", e),
        }
    }
}

pub fn prompt_for_str(prompt_msg: &str) -> String {
    loop {
        let input = prompt(prompt_msg);
        if input.is_empty() {
            println!("Value must not be empty");
            continue;
        }
        return input
    }
}

pub fn read_and_add_invoice_items(connection: &Connection, invoice_id: i64) -> Vec<i64> {
    let mut item_ids = Vec::new();
    loop {
        let description = prompt("Description (leave empty to finish): ");
        if description.is_empty() {
            break;
        }

        let hours = prompt_for_f64("Hours: ");

        let rate = prompt_for_f64("Rate: ");

        let amount = hours * rate;

        // Insert into the database
        connection.execute(
            "INSERT INTO invoice_item (invoice_id, description, hours, rate, amount) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![invoice_id, description, hours, rate, amount],
        ).expect("Failed to insert item");

        // Get the last inserted id
        let item_id = connection.last_insert_rowid();

        item_ids.push(item_id);

        println!("Item added.\n");
    }
    item_ids
}

pub fn generate_pdf(
    invoice: &InvoiceForPdf,
    template: &str,
) -> Result<String, Box<dyn Error>> {
    let html = fs::read_to_string(template)?;

    let date = NaiveDate::parse_from_str(&invoice.date, "%Y-%m-%d")?;
    let due_date = date + Duration::days(30); // Net 30
    let due_date_str = due_date.format("%Y-%m-%d").to_string();

    let invoice_number = format!("INV-{}-{}", invoice.date, invoice.id);

    let year_month = date.format("%Y-%m").to_string();
    let day = date.format("%d").to_string();
    let sanitized_client_name = invoice.client_name.replace("/", "-").replace(" ", "-");
    let filename = format!("{}-{}-{}.pdf", day, invoice.id, sanitized_client_name);
    let folder_path = Path::new("invoices").join(&year_month);
    fs::create_dir_all(&folder_path)?;
    let pdf_path = folder_path.join(&filename);
    let pdf_path_str = pdf_path.to_str().unwrap();

    // Replace template placeholders with invoice values
    let mut filled_template = html
        .replace("{invoice_id}", &invoice_number)
        .replace("{client_name}", &invoice.client_name)
        .replace("{client_email}", &invoice.client_email)
        .replace("{client_phone_number}", &invoice.client_phone_number)
        .replace("{date}", &invoice.date)
        .replace("{due_date}", &due_date_str);

    // Generate item list as text
    let num_items = invoice.items.len() - 1;
    let mut cur_item = 0;
    let mut total_cost = 0.00;
    let items_text: String = invoice
        .items
        .iter()
        .map(|item| {
            if cur_item == num_items {
                cur_item += 1;
                total_cost += item.amount;
                format!(
                    "<tr class=\"item last\"><td>{}</td><td style=\"text-align: left\">{}</td><td style=\"text-align: right;\">${:.2}</td><td style=\"text-align: right;\">${:.2}</td></tr>",
                    item.description, item.hours, item.rate, item.amount
                )
            } else {
                cur_item += 1;
                total_cost += item.amount;
                format!(
                    "<tr class=\"item last\"><td>{}</td><td style=\"text-align: left\">{}</td><td style=\"text-align: right;\">${:.2}</td><td style=\"text-align: right;\">${:.2}</td></tr>",
                    item.description, item.hours, item.rate, item.amount
                )
            }
        })
        .collect();

    filled_template = filled_template.replace("{items}", &items_text);
    filled_template = filled_template.replace("{total}", &format!("{:.2}", total_cost));
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .orientation(Orientation::Portrait)
        .title("Invoice")
        .build_from_html(filled_template)
        .expect("failed to build pdf");

    pdfout.save(pdf_path_str).expect("failed to save file");

    Ok(pdf_path_str.to_string())
}
