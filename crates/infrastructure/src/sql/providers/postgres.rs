use std::time::Duration;
use async_trait::async_trait;
use sqlx::{PgPool};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use conservatory_model::providers::sql::SQLProvider;
use crate::sql::di::unit_of_work::PgUnitOfWork;

#[derive(Debug, Clone)]
pub struct PostgresqlProvider {
        pool: PgPool
}

impl PostgresqlProvider {
        pub async fn new(host: &str, port: u16, username: &str, password: Option<&str>, database: &str)
                         -> Result<Self, anyhow::Error> {
                let mut pg_connect_options =
                        PgConnectOptions::new()
                                .ssl_mode(PgSslMode::Disable)
                                .host(host)
                                .port(port)
                                .username(username)
                                .database(database);

                if let Some(password) = password {
                        pg_connect_options = pg_connect_options.password(password);
                }

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
}

#[async_trait]
impl SQLProvider for PostgresqlProvider {
        type UnitOfWork<'a> = PgUnitOfWork<'a>;

        async fn init(&self) -> Result<(), anyhow::Error> {
                sqlx::migrate!().run(&self.pool).await?;
                
                Ok(())
        }

        async fn begin(&self) -> Result<PgUnitOfWork<'_>, anyhow::Error> {
                let tx = self.pool.begin().await?;

                Ok(PgUnitOfWork { tx })
        }
}