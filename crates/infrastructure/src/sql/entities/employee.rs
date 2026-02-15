use std::str::FromStr;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::time::Date;
use conservatory_model::employee::EmployeeModel;
use uuid::Uuid;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::Currency;

#[derive(Debug)]
pub struct EmployeeEntity(pub EmployeeModel);

impl<'r> FromRow<'r, PgRow> for EmployeeEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let id = row.try_get::<Uuid, _>("id")?.into();
                let name = row.try_get::<String, _>("name")?;
                let surname = row.try_get::<String, _>("surname")?;
                let patronymic = row.try_get::<Option<String>, _>("patronymic")?;
                let currency = Currency::from_str(row.try_get::<&str, _>("currency")?).unwrap();
                let amount = row.try_get::<i32, _>("amount")? as u32;
                let works_since = row.try_get::<Date, _>("works_since")?;

                let salary = Salary {
                        currency,
                        amount
                };

                let employee = EmployeeModel {
                        id,
                        name,
                        surname,
                        patronymic,
                        salary,
                        works_since
                };

                Ok(Self(employee))
        }
}