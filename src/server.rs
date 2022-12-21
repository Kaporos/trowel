use color_eyre::Result;
use clap::Args;
use log::info;

#[derive(Args, Debug)]
pub struct ServerArgs {
}
pub fn main(_args: &ServerArgs) -> Result<()> {
    info!("Running server");
    Ok(())
}