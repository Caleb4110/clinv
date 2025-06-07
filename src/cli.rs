use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Raw command input for natural language processing
    #[clap(flatten)]
    pub raw_command: RawCommandInput,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new client
    NewClient {
        /// Name of the client
        #[arg(short, long)]
        name: Option<String>,

        /// Email of the client
        #[arg(short, long)]
        email: Option<String>,

        /// Phone number of the client
        #[arg(short, long)]
        phone_number: Option<String>,
    },

    /// Create a new invoice
    NewInvoice {
        /// Client name
        #[arg(short, long)]
        client_name: Option<String>,
    },

    /// List all clients
    ListClients,

    /// List all invoices
    ListInvoices {
        /// Client name
        #[arg(short, long)]
        client_name: Option<String>,
    },

    /// Delete a client
    DeleteClient {
        /// Client name
        #[arg(short, long)]
        client_name: Option<String>,
    },

    /// Delete an invoice
    DeleteInvoice {
        /// Filter by invoice ID
        #[arg(short, long)]
        invoice_id: Option<String>,
    },

    /// Generate a PDF for an invoice
    Generate {
        /// invoice ID
        #[arg(short, long)]
        invoice_id: Option<String>,
    },
}

#[derive(Parser, Debug)]
pub struct RawCommandInput {
    /// The raw command words (only used if no structured command is provided)
    #[arg(trailing_var_arg = true)]
    pub words: Vec<String>,
}

/// Maps natural language commands to structured commands
pub fn map_command_words(words: &[String]) -> Option<Commands> {
    if words.is_empty() {
        return None;
    }

    match words {
        [s1, s2, ..] if s1.to_lowercase() == "new" && s2.to_lowercase() == "client" => {
            Some(Commands::NewClient {
                name: None,
                email: None,
                phone_number: None,
            })
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "new" && s2.to_lowercase() == "invoice" => {
            let name = rest.get(0).map(|s| s.clone());
            Some(Commands::NewInvoice { client_name: name })
        }
        [s1, s2] if s1.to_lowercase() == "list" && s2.to_lowercase() == "clients" => {
            Some(Commands::ListClients)
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "list" && s2.to_lowercase() == "invoices" => {
            let name = rest.get(0).map(|s| s.clone());
            Some(Commands::ListInvoices { client_name: name })
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "client" => {
            let name = rest.get(0).map(|s| s.clone());
            Some(Commands::DeleteClient { client_name: name })
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "invoice" => {
            let id = rest.get(0).map(|s| s.clone());
            Some(Commands::DeleteInvoice { invoice_id: id })
        }
        [s1, rest @ ..] if s1.to_lowercase() == "generate" => {
            let id = rest.get(0).map(|s| s.clone());
            Some(Commands::Generate { invoice_id: id })
        }
        _ => None,
    }
}
