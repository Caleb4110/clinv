use clinv::models::{Client, Invoice, InvoiceForPdf, InvoiceItem};

#[test]
fn test_client_struct() {
    let client = Client {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        phone_number: "123456789".to_string(),
    };

    assert_eq!(client.id, 1);
    assert_eq!(client.name, "John Doe");
    assert_eq!(client.email, "john@example.com");
    assert_eq!(client.phone_number, "123456789");
}

#[test]
fn test_invoice_struct() {
    let item = InvoiceItem {
        id: 2,
        description: "Design".to_string(),
        hours: 2.0,
        rate: 150.0,
        amount: 300.0,
    };
    let invoice = Invoice {
        id: 5,
        client_id: 1,
        date: "2025-01-01".to_string(),
        items: vec![item],
    };

    assert_eq!(invoice.id, 5);
    assert_eq!(invoice.client_id, 1);
    assert_eq!(invoice.date, "2025-01-01");
    assert_eq!(invoice.items.len(), 1);
    assert_eq!(invoice.items[0].amount, 300.0);
}

#[test]
fn test_invoice_for_pdf_struct() {
    let items = vec![InvoiceItem {
        id: 3,
        description: "Dev Work".to_string(),
        hours: 3.0,
        rate: 200.0,
        amount: 600.0,
    }];
    let pdf = InvoiceForPdf {
        id: 7,
        client_name: "Jane Smith".to_string(),
        client_email: "jane@smith.com".to_string(),
        client_phone_number: "999888777".to_string(),
        date: "2025-06-06".to_string(),
        items,
    };

    assert_eq!(pdf.id, 7);
    assert_eq!(pdf.client_name, "Jane Smith");
    assert_eq!(pdf.client_email, "jane@smith.com");
    assert_eq!(pdf.client_phone_number, "999888777");
    assert_eq!(pdf.date, "2025-06-06");
    assert_eq!(pdf.items.len(), 1);
    assert_eq!(pdf.items[0].description, "Dev Work");
}

#[test]
fn test_invoice_item_struct() {
    let item = InvoiceItem {
        id: 10,
        description: "Consulting".to_string(),
        hours: 5.0,
        rate: 100.0,
        amount: 500.0,
    };

    assert_eq!(item.id, 10);
    assert_eq!(item.description, "Consulting");
    assert_eq!(item.hours, 5.0);
    assert_eq!(item.rate, 100.0);
    assert_eq!(item.amount, 500.0);
}
