use std::str::FromStr;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::enums::WorkType;

#[derive(Debug)]
pub struct EmployeePlantWorkEntity(pub EmployeePlantWorkModel);

impl<'r> FromRow<'r, PgRow> for EmployeePlantWorkEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let id = row.try_get::<Uuid, _>("id")?.into();
                let employee_id = row.try_get::<Uuid, _>("employee_id")?.into();
                let plant_id = row.try_get::<Uuid, _>("plant_id")?.into();
                let work_type = WorkType::from_str(row.try_get::<&str, _>("work_type")?).unwrap();

                Ok(Self(EmployeePlantWorkModel::new(
                        id, employee_id, plant_id, work_type
                )))
        }
}