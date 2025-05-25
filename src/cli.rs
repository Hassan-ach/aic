// use crate::commands::ask;
use crate::commands::run;
use crate::core::workflow;
use clap::{Parser, Subcommand};
// use tokio;

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
        commands: Vec<String>,

        /// without permitions
        #[arg(short, default_value_t = false)]
        y: bool,

        ///verbos moode
        #[arg(short, long, default_value_t = true)]
        verbose: bool,
    },
    // Ask {
    //     /// The Prompt to sed it to LLM
    //     prompt: String,
    // },
}

pub async fn run_cli(cli: Cli) {
    let mut tasks_pool = workflow::TasksPool::new();
    match cli.command {
        Commands::Run {
            commands,
            y,
            verbose,
        } => {
            for ele in commands {
                tasks_pool.add(workflow::Task::new(
                    0,
                    tokio::spawn(async move {
                        //
                        run::handle_command_execution(&ele, y, verbose).await;
                    }),
                ));
            }
        } // Commands::Ask { prompt } => {
          //     //
          //     // ask::send_request(&prompt);
          // }
    }

    tasks_pool.join().await;
}
