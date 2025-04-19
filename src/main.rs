use async_template::cli::Cli;
use clap::Parser;
use color_eyre::Result;

use crate::app::App;

mod app;
mod async_template;
mod components;
mod frontend;
mod shared_data;
mod sorting;

#[tokio::main]
async fn main() -> Result<()> {
    crate::async_template::errors::init()?;
    crate::async_template::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
