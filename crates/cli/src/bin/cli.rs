use std::thread::Builder;
use conservatory_cli::opt::ConservatoryCli;
use clap::Parser;
use bollard::Docker;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
        simple_logger::init_with_level(log::Level::Info)?;

        if let Err(e) = conservatory_cli::run(ConservatoryCli::parse()).await {
                log::error!("Error executing Conservatory CLI: {e}");
        }

        Ok(())
}