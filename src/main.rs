use color_eyre::Result;
use clap::{Parser, Subcommand};
use log::LevelFilter;
mod client;
mod server;

extern crate pretty_env_logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Client(client::ClientArgs),
    Server(server::ServerArgs)
}



fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_builder().filter_level(LevelFilter::max()).try_init()?;
    let args = Cli::parse();
    match &args.command {
        Commands::Client(args) => client::main(args),
        Commands::Server(args) => server::main(args)
    }
}
