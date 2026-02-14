use crate::db::postgres::run_database_command;
use crate::opt::{Commands, ConservatoryCli};

pub mod opt;
mod db;

pub async fn run(opt: ConservatoryCli) -> anyhow::Result<()> {
        match opt.command {
                Commands::Database(database_opt) => run_database_command(database_opt).await?
        }
        Ok(())
}