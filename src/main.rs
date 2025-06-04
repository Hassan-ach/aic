mod cli;
mod commands;
mod core;
mod utils;
use clap::Parser;
// use tokio;
// use cli;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args = cli::Cli::parse();
    
    cli::run_cli(args).await;
    // println!("{:#?}", args);
}
