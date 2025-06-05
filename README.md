# CLINV (Command Line Invoicer)

This program allows you to create invoices for clients directly from the command line.

## Usage
### Creating invoices/clients
```bash
clinv new [invoice OR client]
```

It will then prompt you for client or invoice information depending on which command you run.

### Listing invoices/clients
```bash
clinv list [invoices OR clients]
```

This will list all stored invoices or clients.

### Deleting invoices/clients
```bash
clinv delete [invoice OR client]
```

This will delete an invoice or client after you give it the invoice/client ID

### Generating a PDF
```bash
clinv generate
```

This will generate a PDF once you give it the invoice ID

## Notes
Still a WIP
