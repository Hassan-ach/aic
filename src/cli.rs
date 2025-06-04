use crate::commands::ask;
use crate::commands::chat;
use crate::commands::run;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[command(name = "aic")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a command directly
    Run {
        /// The command to execute
        #[arg(required = true)]
        command: String,

        /// without permitions
        #[arg(short, default_value_t = false)]
        y: bool,
    },
    Ask {
        /// The Prompt to sed it to LLM
        #[arg(required = true)]
        prompt: String,

        /// without permitions
        #[arg(short, default_value_t = false)]
        y: bool,
    },
    Chat {},
}

pub async fn run_cli(cli: Cli) {
    match cli.command {
        Commands::Run { command, y } => {
            run::handle_command_execution(&command, y);
        }
        Commands::Ask { prompt, y } => {
            ask::handle_prompt_req(&prompt, y)
                .await
                .unwrap_or_else(|e| eprintln!("{e}"));
        }
        Commands::Chat {} => {
            chat::chat().await.unwrap_or_else(|e| eprintln!("{e}"));
        }
    }
}
