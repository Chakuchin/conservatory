use std::ops::Deref;
use anyhow::Error;
use sqlx::PgConnection;
use conservatory_core::id::{Id, TypeId};
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::enums::WorkType;
use conservatory_model::plant::{PlantModel, PlantTypeModel};
use conservatory_model::repositories::sql::plant::PlantRepository;
use crate::sql::entities::employee::EmployeeEntity;
use crate::sql::entities::employee_plant_work::EmployeePlantWorkEntity;
use crate::sql::entities::plant::PlantEntity;
use crate::sql::entities::plant_type::PlantTypeEntity;
use crate::sql::entities::plant_with_type::PlantWithTypeJoinEntity;
use crate::sql::repositories::employee::EmployeePostgresqlRepository;

#[derive(Debug)]
pub struct PlantPostgresqlRepository<'a> {
        pub connection: &'a mut PgConnection
}

impl<'a> PlantPostgresqlRepository<'a> {
        pub fn new(connection: &'a mut PgConnection) -> Self {
                Self {
                        connection
                }
        }
}

#[async_trait::async_trait]
impl<'a> PlantRepository for PlantPostgresqlRepository<'a> {
        async fn register(&mut self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, Error> {
                let plant_type: PlantTypeEntity = sqlx::query_as("
                        INSERT INTO \"plant_type\" (urn, name, description)
                        VALUES ($1, $2, $3)
                        RETURNING urn, name, description
                        ")
                        .bind(plant_type.urn.as_uuid())
                        .bind(&plant_type.name)
                        .bind(&plant_type.description)
                        .fetch_one(self.connection.as_mut())
                        .await?;

                Ok(plant_type.0)
        }

        async fn create(&mut self, id: &Id, type_urn: &TypeId) -> Result<PlantModel, Error> {
                let plant: PlantWithTypeJoinEntity = sqlx::query_as("
                        WITH new_plant AS (
                                INSERT INTO \"plant\" (id, type_urn)
                                VALUES ($1, $2)
                                RETURNING id, type_urn, greenhouse_id
                        )
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM new_plant p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        ")
                        .bind(id.deref())
                        .bind(type_urn.deref().as_uuid())
                        .fetch_one(self.connection.as_mut())
                        .await?;

                Ok(plant.into_plant_model())
        }

        async fn get_type(&mut self, type_urn: &TypeId) -> Result<Option<PlantTypeModel>, Error> {
                let plant_type: Option<PlantTypeEntity> = sqlx::query_as("
                        SELECT urn, name, description
                        FROM \"plant_type\"
                        WHERE urn = $1
                        ")
                        .bind(type_urn.as_uuid())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant_type.map(|inner| inner.0))
        }

        async fn get(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let plant: Option<PlantWithTypeJoinEntity> = sqlx::query_as("
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM \"plant\" p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        WHERE p.id = $1 AND p.deleted_at IS NULL
                        ")
                        .bind(plant_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant.map(|inner| inner.into_plant_model()))
        }

        async fn list_types(&mut self) -> Result<Vec<PlantTypeModel>, Error> {
                let plant_types: Vec<PlantTypeEntity> = sqlx::query_as("
                        SELECT urn, name, description
                        FROM \"plant_type\"
                        ")
                        .fetch_all(self.connection.as_mut())
                        .await?;

                Ok(plant_types.into_iter().map(|inner| inner.0).collect())
        }

        async fn list(&mut self) -> Result<Vec<PlantModel>, Error> {
                let plants: Vec<PlantWithTypeJoinEntity> = sqlx::query_as("
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM \"plant\" p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        WHERE p.deleted_at IS NULL
                        ")
                        .fetch_all(self.connection.as_mut())
                        .await?;

                Ok(plants.into_iter().map(|inner| inner.into_plant_model()).collect())
        }

        async fn update_type_description(&mut self, type_urn: &TypeId, description: &str) -> Result<Option<PlantTypeModel>, Error> {
                todo!()
        }

        async fn soft_delete_plant(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let plant: Option<PlantWithTypeJoinEntity> = sqlx::query_as("
                        WITH d_plant AS (
                                UPDATE \"plant\"
                                SET deleted_at = CURRENT_TIMESTAMP
                                WHERE id = $1
                                RETURNING id, type_urn, greenhouse_id
                        )
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM d_plant p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        ")
                        .bind(plant_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant.map(|inner| inner.into_plant_model()))
        }

        async fn delete_plant(&mut self, plant_id: &Id) -> Result<Option<()>, Error> {
                let plant: Option<()> = sqlx::query_as("
                        DELETE FROM \"plant\"
                        WHERE id = $1
                        ")
                        .bind(plant_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant)
        }

        async fn delete_type(&mut self, type_urn: &TypeId) -> Result<Option<()>, Error> {
                let plant: Option<()> = sqlx::query_as("
                        DELETE FROM \"plant_type\"
                        WHERE urn = $1
                        ")
                        .bind(type_urn.deref().as_uuid())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant)
        }

        async fn restore_plant(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let plant: Option<PlantWithTypeJoinEntity> = sqlx::query_as("
                        WITH d_plant AS (
                                UPDATE \"plant\"
                                SET deleted_at = NULL
                                WHERE id = $1
                                RETURNING id, type_urn, greenhouse_id
                        )
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM d_plant p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        ")
                        .bind(plant_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant.map(|inner| inner.into_plant_model()))
        }

        async fn work_with(&mut self, plant_id: &Id, employee_id: &Id, work_type: &WorkType) -> Result<Option<EmployeePlantWorkModel>, Error> {
                let work: Option<EmployeePlantWorkEntity> = sqlx::query_as("
                        INSERT INTO \"employee_plant_work\" (id, employee_id, plant_id, work_type)
                        VALUES ($1, $2, $3, $4::WORK_TYPE)
                        RETURNING id, employee_id, plant_id, work_type
                        ")
                        .bind(Id::new().deref())
                        .bind(employee_id.deref())
                        .bind(plant_id.deref())
                        .bind(work_type.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(work.map(|inner| inner.0))
        }

        async fn plant_at(&mut self, plant_id: &Id, greenhouse_id: &Id) -> Result<Option<PlantModel>, Error> {
                let plant: Option<PlantWithTypeJoinEntity> = sqlx::query_as("
                        WITH d_plant AS (
                                UPDATE \"plant\"
                                SET greenhouse_id = $2
                                WHERE id = $1 AND deleted_at IS NULL
                                RETURNING id, type_urn, greenhouse_id
                        )
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM d_plant p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        ")
                        .bind(plant_id.deref())
                        .bind(greenhouse_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant.map(|inner| inner.into_plant_model()))
        }

        async fn remove_from(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let plant: Option<PlantWithTypeJoinEntity> = sqlx::query_as("
                        WITH d_plant AS (
                                UPDATE \"plant\"
                                SET greenhouse_id = NULL
                                WHERE id = $1 AND deleted_at IS NULL
                                RETURNING id, type_urn, greenhouse_id
                        )
                        SELECT p.id, t.urn, t.name, t.description, p.greenhouse_id
                        FROM d_plant p
                        JOIN \"plant_type\" t ON p.type_urn = t.urn
                        ")
                        .bind(plant_id.deref())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(plant.map(|inner| inner.into_plant_model()))
        }
}