use clap::{Parser, Subcommand};
use color_eyre::Result;
use log::LevelFilter;
use client;
use server;

extern crate pretty_env_logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Client(client::ClientArgs),
    Server(server::ServerArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .try_init()?;
    let args = Cli::parse();
    match &args.command {
        Commands::Client(args) => client::launch(args).await,
        Commands::Server(args) => server::launch(args).await,
    }
}
