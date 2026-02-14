use anyhow::Error;
use sqlx::PgTransaction;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::identities::employee_id::EmployeeId;
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
                let employee: Option<EmployeeEntity>  = sqlx::query_as(
                        "SELECT id, name, surname, patronymic, amount::INT4, currency::TEXT, works_since \
                        FROM \"employee\" \
                        WHERE id = $1::uuid"
                        )
                        .bind(id.to_string())
                        .fetch_optional(self.transaction.as_mut())
                        .await?;

                Ok(employee.map(|inner| inner.0))
        }
}