use anyhow::Error;
use sqlx::PgTransaction;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::id::EmployeeId;
use conservatory_model::employee::salary::Salary;
use conservatory_model::repositories::employee::EmployeeRepository;
use crate::sql::entities::employee::EmployeeEntity;

#[derive(Debug)]
pub struct EmployeePostgresqlRepository<'a, 'ts> {
        pub transaction: &'a mut PgTransaction<'ts>
}

impl<'a, 'ts> EmployeePostgresqlRepository<'a, 'ts> {
        pub fn new(transaction: &'a mut PgTransaction<'ts>) -> Self {
                Self {
                        transaction
                }
        }
}

impl EmployeeRepository for EmployeePostgresqlRepository<'_, '_> {
        async fn create(&mut self, employee: EmployeeModel) -> Result<EmployeeModel, Error> {
                let EmployeeEntity(new_employee) = sqlx::query_as(
                        "INSERT INTO \"employee\" (id, name, surname, patronymic, amount, currency, works_since) \
                        VALUES ($1::uuid, $2, $3, $4, $5::U32, $6::CURRENCY, $7) \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(employee.id.to_string())
                        .bind(employee.name)
                        .bind(employee.surname)
                        .bind(employee.patronymic)
                        .bind(employee.salary.amount as i32)
                        .bind(employee.salary.currency.to_string())
                        .bind(employee.works_since)
                        .fetch_one(self.transaction.as_mut())
                        .await?;

                Ok(new_employee)
        }

        async fn get(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "SELECT id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since \
                        FROM \"employee\" \
                        WHERE id = $1::uuid AND deleted_at IS NULL"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn list(&mut self) -> Result<Vec<EmployeeModel>, Error> {
                let employees: Vec<EmployeeEntity> = sqlx::query_as(
                        "SELECT id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since \
                        FROM \"employee\" \
                        WHERE deleted_at IS NULL"
                        )
                        .fetch_all(self.transaction.as_mut())
                        .await?;

                Ok(employees.into_iter().map(|inner| inner.0).collect())
        }

        async fn update_salary(&mut self, id: EmployeeId, salary: Salary) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                        SET amount = $1::U32, currency = $2::CURRENCY \
                        WHERE id = $3::uuid AND deleted_at IS NULL \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(salary.amount.to_string())
                        .bind(salary.currency.to_string())
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn soft_delete(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                                SET deleted_at = CURRENT_TIMESTAMP \
                                WHERE id = $1::uuid AND deleted_at IS NULL \
                                RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                )
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn delete(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "DELETE FROM \"employee\" \
                        WHERE id = $1::uuid \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }

        async fn restore(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, Error> {
                let employee: Option<EmployeeEntity> = sqlx::query_as(
                        "UPDATE \"employee\" \
                        SET deleted_at = NULL \
                        WHERE id = $1::uuid AND deleted_at IS NOT NULL \
                        RETURNING id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }
}