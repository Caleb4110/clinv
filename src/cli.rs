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
        /// Client ID
        #[arg(short, long)]
        client: Option<String>,
    },

    /// List all clients
    ListClients,

    /// List all invoices
    ListInvoices {
        /// Filter by client ID
        #[arg(short, long)]
        client: Option<String>,
    },

    /// Delete a client
    DeleteClient {
        /// Filter by client ID
        #[arg(short, long)]
        client_id: Option<String>,
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
        [s1, s2, ..] if s1.to_lowercase() == "new" && s2.to_lowercase() == "invoice" => {
            Some(Commands::NewInvoice { client: None })
        }
        [s1, s2] if s1.to_lowercase() == "list" && s2.to_lowercase() == "clients" => {
            Some(Commands::ListClients)
        }
        [s1, s2] if s1.to_lowercase() == "list" && s2.to_lowercase() == "invoices" => {
            Some(Commands::ListInvoices { client: None })
        }
        [s1, s2] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "client" => {
            Some(Commands::DeleteClient { client_id: None })
        }
        [s1, s2] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "invoice" => {
            Some(Commands::DeleteInvoice { invoice_id: None })
        }
        [s1, ..] if s1.to_lowercase() == "generate" => {
            Some(Commands::Generate { invoice_id: None })
        }
        _ => None,
    }
}
