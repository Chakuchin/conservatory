use std::time::Duration;
use sqlx::{ConnectOptions, PgPool, PgTransaction};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use crate::sql::di::unit_of_work::PgUnitOfWork;
use crate::sql::repositories::employee::EmployeePostgresqlRepository;

#[derive(Debug)]
pub struct PostgresqlProvider {
        pool: PgPool
}

impl PostgresqlProvider {
        pub async fn new(host: &str, port: u16, username: &str, database: &str)
                -> Result<Self, anyhow::Error> {
                let pg_connect_options =
                        PgConnectOptions::new()
                                .ssl_mode(PgSslMode::Disable)
                                .host(host)
                                .port(port)
                                .username(username)
                                .database(database);

                let pool =
                        PgPoolOptions::new()
                                .min_connections(5)
                                .max_connections(20)
                                .acquire_timeout(Duration::from_secs(20))
                                .idle_timeout(Some(Duration::from_secs(600)))
                                .max_lifetime(None)
                                .connect_with(pg_connect_options)
                                .await?;

                Ok(Self { pool })
        }

        pub async fn init(&self) -> Result<(), anyhow::Error> {
                sqlx::migrate!("../../migrations").run(&self.pool).await?;

                Ok(())
        }

        pub async fn begin(&self) -> Result<PgUnitOfWork<'_>, anyhow::Error> {
                let tx = self.pool.begin().await?;

                Ok(PgUnitOfWork { tx })
        }

        pub fn provide(&self) -> PgPool {
                self.pool.clone()
        }
}