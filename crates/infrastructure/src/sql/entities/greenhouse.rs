use std::ops::Deref;
use fixed::FixedU32;
use fixed::types::extra::U2;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_model::greenhouse::GreenhouseModel;

#[derive(Debug)]
pub struct GreenhouseEntity(pub GreenhouseModel);

impl<'r> FromRow<'r, PgRow> for GreenhouseEntity {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let id = row.try_get::<Uuid, _>("id")?.into();
                let name = row.try_get::<&str, _>("name")?;
                let humidity = row.try_get::<i32, _>("humidity")? as RelativeHumidity;
                let temperature = Temperature::<Celsius>::new(row.try_get::<f32, _>("target_temperature_celsius")?.into());
                let area = FixedU32::<U2>::from_num(row.try_get::<f32, _>("area_square_meters")?);
                let conditions = row.try_get::<Vec<String>, _>("conditions")?.into_iter().map(|inner| inner.parse().unwrap()).collect::<Vec<_>>();
                
                Ok(Self (
                        GreenhouseModel::new(id, name, humidity, temperature, conditions.as_slice(), area)
                ))
        }
}