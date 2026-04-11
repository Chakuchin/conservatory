use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;
use conservatory_model::plant::PlantTypeModel;

#[derive(Debug)]
pub struct PlantTypeEntity(pub PlantTypeModel);

impl<'r> FromRow<'r, PgRow> for PlantTypeEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let urn = row.try_get::<Uuid, _>("urn")?;
                let name = row.try_get::<String, _>("name")?;
                let description = row.try_get::<String, _>("description")?;

                Ok(Self(PlantTypeModel::new(
                        urn.urn(), name, description
                )))
        }
}