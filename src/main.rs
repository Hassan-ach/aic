mod cli;
mod commands;
mod core;
use clap::Parser;
// use tokio;
// use cli;
#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();
    cli::run_cli(args).await;
    // println!("{:#?}", args);
}
