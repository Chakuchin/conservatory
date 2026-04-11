use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Debug)]
pub struct PlantEntity {
        pub id: Uuid,
        pub type_urn: Uuid,
        pub greenhouse_id: Option<Uuid>
}

impl<'r> FromRow<'r, PgRow> for PlantEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let id = row.try_get::<Uuid, _>("id")?;
                let type_urn = row.try_get::<Uuid, _>("type_urn")?;
                let greenhouse_id = row.try_get::<Option<Uuid>, _>("greenhouse_id")?;

                Ok(Self {
                        id,
                        type_urn,
                        greenhouse_id
                })
        }
}