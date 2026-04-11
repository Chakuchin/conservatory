use std::ops::Deref;
use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgConnection;
use conservatory_model::employee::EmployeeModel;
use conservatory_core::id::Id;
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::WorkType;
use conservatory_model::repositories::sql::employee::EmployeeRepository;
use crate::sql::entities::employee::EmployeeEntity;
use crate::sql::entities::employee_plant_work::EmployeePlantWorkEntity;

#[derive(Debug)]
pub struct EmployeePostgresqlRepository<'a> {
        pub connection: &'a mut PgConnection
}

impl<'a> EmployeePostgresqlRepository<'a> {
        pub fn new(connection: &'a mut PgConnection) -> Self {
                Self {
                        connection
                }
        }
}

#[async_trait]
impl EmployeeRepository for EmployeePostgresqlRepository<'_> {
        async fn create(&mut self, employee: &EmployeeModel) -> Result<EmployeeModel, Error> {
                let EmployeeEntity(new_employee) = sqlx::query_as(
                        "INSERT INTO \"employee\" (id, name, surname, patronymic, amount, currency, works_since) \
                        VALUES ($1::uuid, $2, $3, $4, $5::UINT, $6::CURRENCY, $7) \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(employee.id.to_string())
                        .bind(employee.name.clone())
                        .bind(employee.surname.clone())
                        .bind(employee.patronymic.clone())
                        .bind(employee.salary.amount as i32)
                        .bind(employee.salary.currency.to_string())
                        .bind(employee.works_since)
                        .fetch_one(self.connection.as_mut())
                        .await?;

                Ok(new_employee)
        }

        async fn get(&mut self, id: &Id) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "SELECT id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since \
                        FROM \"employee\" \
                        WHERE id = $1::uuid AND deleted_at IS NULL"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn list(&mut self) -> Result<Vec<EmployeeModel>, Error> {
                let employees: Vec<EmployeeEntity> = sqlx::query_as(
                        "SELECT id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since \
                        FROM \"employee\" \
                        WHERE deleted_at IS NULL"
                        )
                        .fetch_all(self.connection.as_mut())
                        .await?;

                Ok(employees.into_iter().map(|inner| inner.0).collect())
        }

        async fn update_salary(&mut self, id: &Id, salary: &Salary) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                        SET amount = $1::UINT, currency = $2::CURRENCY, updated_at = CURRENT_TIMESTAMP \
                        WHERE id = $3::uuid AND deleted_at IS NULL \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(salary.amount.to_string())
                        .bind(salary.currency.to_string())
                        .bind(id.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn soft_delete(&mut self, id: &Id) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                                SET deleted_at = CURRENT_TIMESTAMP \
                                WHERE id = $1::uuid AND deleted_at IS NULL \
                                RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                )
                        .bind(id.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn delete(&mut self, id: &Id) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "DELETE FROM \"employee\" \
                        WHERE id = $1::uuid \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn restore(&mut self, id: &Id) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                        SET deleted_at = NULL \
                        WHERE id = $1::uuid AND deleted_at IS NOT NULL \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.connection.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }
}