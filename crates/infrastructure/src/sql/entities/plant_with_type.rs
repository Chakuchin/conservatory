use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;
use conservatory_model::plant::{PlantModel, PlantTypeModel};

#[derive(Debug)]
pub struct PlantWithTypeJoinEntity {
        plant_id: Uuid,
        plant_type_urn: Uuid,
        greenhouse_id: Option<Uuid>,
        plant_type_name: String,
        plant_type_description: String,
}

impl PlantWithTypeJoinEntity {
        pub fn into_plant_model(self) -> PlantModel {
                let plant_type = PlantTypeModel::new(
                        self.plant_type_urn.urn(),
                        self.plant_type_name,
                        self.plant_type_description
                );

                PlantModel::new(
                        self.plant_id.into(),
                        plant_type,
                        self.greenhouse_id.map(Into::into),
                )
        }
}

impl<'r> FromRow<'r, PgRow> for PlantWithTypeJoinEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                Ok(Self {
                        plant_id: row.try_get("id")?,
                        plant_type_urn: row.try_get("urn")?,
                        greenhouse_id: row.try_get("greenhouse_id")?,
                        plant_type_name: row.try_get("name")?,
                        plant_type_description: row.try_get("description")?,
                })
        }
}