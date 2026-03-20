use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgTransaction;
use uuid::fmt::Urn;
use conservatory_model::employee::EmployeeModel;
use conservatory_core::id::Id;
use conservatory_model::employee::salary::Salary;
use conservatory_model::plant_type::PlantTypeModel;
use conservatory_model::repositories::sql::plant_type::PlantTypeRepository;
use crate::sql::entities::employee::EmployeeEntity;
use crate::sql::entities::plant_type::PlantTypeEntity;

#[derive(Debug)]
pub struct PlantTypePostgresqlRepository<'a, 'ts> {
        pub transaction: &'a mut PgTransaction<'ts>
}

impl<'a, 'ts> PlantTypePostgresqlRepository<'a, 'ts> {
        pub fn new(transaction: &'a mut PgTransaction<'ts>) -> Self {
                Self {
                        transaction
                }
        }
}

#[async_trait]
impl PlantTypeRepository for PlantTypePostgresqlRepository<'_, '_> {
        async fn create(&mut self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, Error> {
                let PlantTypeEntity(new_plant_type) = sqlx::query_as(
                        "INSERT INTO \"plant_type\" (urn, name, description) \
                        VALUES ($1::uuid, $2, $3) \
                        RETURNING urn, name, description"
                )
                        .bind(plant_type.urn.as_uuid())
                        .bind(&plant_type.name)
                        .bind(&plant_type.description)
                        .fetch_one(self.transaction.as_mut())
                        .await?;

                Ok(new_plant_type)
        }

        async fn get(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as(
                        "SELECT urn, name, description \
                        FROM \"plant_type\" \
                        WHERE urn = $1::uuid AND deleted_at IS NULL"
                )
                        .bind(urn.as_uuid())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }

        async fn list(&mut self) -> Result<Vec<PlantTypeModel>, Error> {
                let plant_types: Vec<PlantTypeEntity> = sqlx::query_as(
                        "SELECT urn, name, description \
                        FROM \"plant_type\" \
                        WHERE deleted_at IS NULL"
                )
                        .fetch_all(self.transaction.as_mut())
                        .await?;

                Ok(plant_types.into_iter().map(|inner| inner.0).collect())
        }

        async fn update_description(&mut self, urn: &Urn, description: &str) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as(
                        "UPDATE \"plant_type\" \
                        SET description = $1, updated_at = CURRENT_TIMESTAMP \
                        WHERE urn = $2::uuid AND deleted_at IS NULL \
                        RETURNING urn, name, description"
                )
                        .bind(description)
                        .bind(urn.as_uuid())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }

        async fn soft_delete(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as(
                        "UPDATE \"plant_type\" \
                                SET deleted_at = CURRENT_TIMESTAMP \
                                WHERE urn = $1::uuid AND deleted_at IS NULL \
                                RETURNING urn, name, description"
                )
                        .bind(urn.as_uuid())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }

        async fn delete(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as(
                        "DELETE FROM \"plant_type\" \
                        WHERE urn = $1::uuid \
                        RETURNING urn, name, description"
                )
                        .bind(urn.as_uuid())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }

        async fn restore(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as(
                        "UPDATE \"plant_type\" \
                        SET deleted_at = NULL \
                        WHERE urn = $1::uuid AND deleted_at IS NOT NULL \
                        RETURNING urn, name, description"
                )
                        .bind(urn.as_uuid())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }
}