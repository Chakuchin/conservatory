use clap::Parser;
use strum::{Display, EnumString, AsRefStr};

#[derive(Debug, Parser)]
#[clap(version, about, author)]
pub struct ConservatoryCli {
        #[clap(subcommand)]
        pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
        Database(DatabaseOpt)
}

#[derive(Parser, Debug)]
#[command(about = "Postgres database operations", long_about = None)]
pub struct DatabaseOpt {
        #[clap(subcommand)]
        pub command: DatabaseCommand,
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseConfig {
        #[arg(long)]
        pub host: String,
        #[arg(long)]
        pub port: u16,
        #[arg(long)]
        pub username: String,
        #[arg(long)]
        pub database: String
}

#[derive(Parser, Debug, Clone)]
#[command(about = "Postgres database operations", long_about = None)]
pub enum DatabaseCommand {
        Init(DatabaseConfig),
}

#[derive(
        Debug,
        Display,
        EnumString,
        AsRefStr,
        Clone,
        Hash,
        Eq,
        PartialEq
)]
#[strum(ascii_case_insensitive)]
pub enum RuntimeCommand {
        Create,
        Get,
        List,
        Update,
        Delete,
        Exit,
        Help,
        #[strum(default)]
        Unknown(String)
}