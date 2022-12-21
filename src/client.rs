use color_eyre::{Result};
use clap::Args;
use log::{info};

#[derive(Args, Debug)]
pub struct ClientArgs {
}
pub fn main(_args: &ClientArgs) -> Result<()> {
    info!("Running client");
    //Err(eyre!("Something happened !"))
    Ok(())
}