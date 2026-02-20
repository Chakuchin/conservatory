use conservatory_cli::opt::ConservatoryCli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
        simple_logger::init_with_level(log::Level::Info)?;

        match conservatory_cli::init(ConservatoryCli::parse()).await {
                Ok(db) => conservatory_cli::run(db).await,
                Err(e) => {
                        log::error!("Error executing Conservatory CLI: {e}");

                        Ok(())
                }
        }
}