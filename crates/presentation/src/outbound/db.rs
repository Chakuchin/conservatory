use std::net::Ipv4Addr;
use std::sync::LazyLock;
use conservatory_infrastructure::sql::providers::postgres::PostgresqlProvider;
use conservatory_model::providers::sql::SQLProvider;

static DB_HOST: LazyLock<String> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "DB_HOST",
                Ipv4Addr::LOCALHOST.to_string()
        )
);

static DB_PORT: LazyLock<u16> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "DB_PORT",
                5432
        )
);

static DB_USER: LazyLock<String> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "DB_USER",
                String::from("test")
        )
);

static DB_PASSWORD: LazyLock<Option<String>> = LazyLock::new(||
        conservatory_core::env::resolve_opt(
                "DB_PASSWORD"
        )
);

static DB_NAME: LazyLock<String> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "DB_NAME",
                String::from("test")
        )
);

pub async fn init() -> PostgresqlProvider {
        let db = PostgresqlProvider::new(&DB_HOST, *DB_PORT, &DB_USER, DB_PASSWORD.as_deref(), &DB_NAME).await.unwrap();
        db.init().await.unwrap();

        db
}