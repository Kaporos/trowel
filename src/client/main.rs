use clap::Args;
use color_eyre::Result;
use log::info;

#[derive(Args, Debug)]
pub struct ClientArgs {}
pub async fn main(_args: &ClientArgs) -> Result<()> {
    info!("Running client");

    Ok(())
}