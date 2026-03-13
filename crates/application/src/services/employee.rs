use async_trait::async_trait;
use conservatory_model::di::unit_of_work::UnitOfWork;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::id::EmployeeId;
use conservatory_model::employee::salary::Salary;
use conservatory_model::providers::sql::SQLProvider;
use conservatory_model::repositories::sql::employee::EmployeeRepository;
use conservatory_model::services::employee::EmployeeService;

#[derive(Debug)]
pub struct BaseEmployeeService<DB: SQLProvider> {
        db_provider: DB
}

impl<DB: SQLProvider> BaseEmployeeService<DB> {
        pub fn new(db_provider: DB) -> Self {
                Self { db_provider }
        }
}

#[async_trait]
impl<DB: SQLProvider> EmployeeService for BaseEmployeeService<DB> {
        async fn create(&self, employee: &EmployeeModel) -> Result<EmployeeModel, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let new_employee = {
                        let mut repo = unit_of_work.employee_repo();
                        repo.create(employee).await?
                };

                unit_of_work.commit().await?;

                Ok(new_employee)
        }

        async fn get(&self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let employee = {
                        let mut repo = unit_of_work.employee_repo();
                        repo.get(id).await?
                };

                unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn list(&self) -> Result<Vec<EmployeeModel>, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let employees = {
                        let mut repo = unit_of_work.employee_repo();
                        repo.list().await?
                };

                unit_of_work.commit().await?;

                Ok(employees)
        }

        async fn update_salary(&self, id: &EmployeeId, salary: &Salary) -> Result<Option<EmployeeModel>, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let employee = {
                        let mut repo = unit_of_work.employee_repo();
                        repo.update_salary(id, salary).await?
                };

                unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn delete(&self, id: &EmployeeId, is_soft: bool) -> Result<Option<EmployeeModel>, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let employee = {
                        let mut repo = unit_of_work.employee_repo();
                        if is_soft {
                                repo.soft_delete(id).await?
                        }
                        else {
                                repo.delete(id).await?
                        }
                };

                unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn restore(&self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error> {
                let mut unit_of_work = self.db_provider.begin().await?;
                let employee = {
                        let mut repo = unit_of_work.employee_repo();
                        repo.restore(id).await?
                };

                unit_of_work.commit().await?;

                Ok(employee)
        }
}