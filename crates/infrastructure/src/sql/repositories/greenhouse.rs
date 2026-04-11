use std::ops::Deref;
use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgConnection;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use conservatory_model::enums::Condition;
use conservatory_model::greenhouse::GreenhouseModel;
use conservatory_model::repositories::sql::greenhouse::GreenhouseRepository;
use crate::sql::entities::greenhouse::GreenhouseEntity;

#[derive(Debug)]
pub struct GreenhousePostgresqlRepository<'a> {
        connection: &'a mut PgConnection
}

impl<'a> GreenhousePostgresqlRepository<'a> {
        pub fn new(connection: &'a mut PgConnection) -> Self {
                Self {
                        connection
                }
        }
}

#[async_trait::async_trait]
impl<'a> GreenhouseRepository for GreenhousePostgresqlRepository<'a> {
        async fn create(&mut self, greenhouse: &GreenhouseModel) -> Result<GreenhouseModel, Error> {
                let greenhouse: GreenhouseEntity = sqlx::query_as("
                        INSERT INTO \"greenhouse\"(id, name, humidity, area_square_meters, target_temperature_celsius)
                        VALUES ($1, $2, $3::UINT, $4::UREAL, $5::UREAL)
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius, (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = $1) AS conditions
                        ")
                        .bind(greenhouse.id.deref())
                        .bind(&greenhouse.name)
                        .bind(greenhouse.humidity as i32)
                        .bind(greenhouse.area.to_num::<f32>())
                        .bind(greenhouse.target_temperature.celsius())
                        .fetch_one(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.0)
        }

        async fn get(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        SELECT g.id, g.name, g.humidity, g.area_square_meters, g.target_temperature_celsius,
                                (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = $1) AS conditions
                        FROM \"greenhouse\" g
                        WHERE id = $1 AND deleted_at IS NULL
                        ")
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn list(&mut self) -> Result<Vec<GreenhouseModel>, Error> {
                let greenhouses: Vec<GreenhouseEntity> = sqlx::query_as("
                        SELECT g.id, g.name, g.humidity, g.area_square_meters, g.target_temperature_celsius, (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = g.id) AS conditions
                        FROM \"greenhouse\" g
                        WHERE deleted_at IS NULL
                        ")
                        .fetch_all(self.connection.as_mut())
                        .await?;

                Ok(greenhouses.into_iter().map(|inner| inner.0).collect())
        }

        async fn update_humidity(&mut self, id: &Id, humidity: RelativeHumidity) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        UPDATE \"greenhouse\"
                        SET humidity = $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2 AND deleted_at IS NULL
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius
                        ")
                        .bind(humidity as i32)
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn update_target_temperature(&mut self, id: &Id, temperature: Temperature<Celsius>) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        UPDATE \"greenhouse\"
                        SET target_temperature_celsius = $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2 AND deleted_at IS NULL
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius
                        ")
                        .bind(temperature.celsius())
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn soft_delete(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        UPDATE \"greenhouse\"
                        SET deleted_at = CURRENT_TIMESTAMP
                        WHERE id = $1 AND deleted_at IS NULL
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius, (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = $1) AS conditions
                        ")
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn delete(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        DELETE FROM \"greenhouse\"
                        WHERE id = $1
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius, (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = $1) AS conditions
                        ")
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn restore(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        UPDATE \"greenhouse\"
                        SET deleted_at = NULL
                        WHERE id = $1 AND deleted_at IS NOT NULL
                        RETURNING id, name, humidity, area_square_meters, target_temperature_celsius, (SELECT COALESCE(ARRAY_AGG(c.condition::TEXT), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" c WHERE c.greenhouse_id = $1) AS conditions
                        ")
                        .bind(id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn add_condition(&mut self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        WITH c AS (
                                INSERT INTO \"greenhouse_condition\" (greenhouse_id, condition)
                                VALUES ($1, $2::CONDITION)
                                RETURNING (SELECT COALESCE(ARRAY_AGG(condition::TEXT), ARRAY[]::TEXT[]) || $2 FROM \"greenhouse_condition\" WHERE greenhouse_id = $1) AS conditions
                        )
                        SELECT g.id, g.name, g.humidity, g.area_square_meters, g.target_temperature_celsius, c.conditions
                        FROM c
                        JOIN \"greenhouse\" g ON id = $1
                        WHERE g.deleted_at IS NULL
                        ")
                        .bind(id.deref())
                        .bind(condition.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }

        async fn remove_condition(&mut self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, Error> {
                let greenhouse: Option<GreenhouseEntity> = sqlx::query_as("
                        WITH c AS (
                                DELETE FROM \"greenhouse_condition\"
                                WHERE greenhouse_id = $1 AND condition = $2::CONDITION
                                RETURNING (SELECT COALESCE(ARRAY_REMOVE(ARRAY_AGG(condition::TEXT), $2), ARRAY[]::TEXT[]) FROM \"greenhouse_condition\" WHERE greenhouse_id = $1) AS conditions
                        )
                        SELECT g.id, g.name, g.humidity, g.area_square_meters, g.target_temperature_celsius, c.conditions
                        FROM c
                        JOIN \"greenhouse\" g ON id = $1
                        WHERE g.deleted_at IS NULL
                        ")
                        .bind(id.deref())
                        .bind(condition.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(greenhouse.map(|inner| inner.0))
        }
}