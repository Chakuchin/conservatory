use anyhow::Error;
use conservatory_model::di::unit_of_work::UnitOfWork;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::id::EmployeeId;
use conservatory_model::employee::salary::Salary;
use conservatory_model::repositories::employee::EmployeeRepository;
use conservatory_model::services::employee::EmployeeService;

#[derive(Debug)]
pub struct BaseEmployeeService<U: UnitOfWork> {
        unit_of_work: U
}

impl<U: UnitOfWork> BaseEmployeeService<U> {
        pub fn new(unit_of_work: U) -> Self {
                Self { unit_of_work }
        }
}

impl<U: UnitOfWork> EmployeeService for BaseEmployeeService<U> {
        async fn create(mut self, employee: EmployeeModel) -> Result<EmployeeModel, anyhow::Error> {
                let new_employee = {
                        let mut repo = self.unit_of_work.employee_repo();
                        repo.create(employee).await?
                };

                self.unit_of_work.commit().await?;

                Ok(new_employee)
        }

        async fn get(mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error> {
                let employee = {
                        let mut repo = self.unit_of_work.employee_repo();
                        repo.get(id).await?
                };

                self.unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn list(mut self) -> Result<Vec<EmployeeModel>, Error> {
                let employees = {
                        let mut repo = self.unit_of_work.employee_repo();
                        repo.list().await?
                };

                self.unit_of_work.commit().await?;

                Ok(employees)
        }

        async fn update_salary(mut self, id: EmployeeId, salary: Salary) -> Result<Option<EmployeeModel>, Error> {
                let employee = {
                        let mut repo = self.unit_of_work.employee_repo();
                        repo.update_salary(id, salary).await?
                };

                self.unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn delete(mut self, id: EmployeeId, is_soft: bool) -> Result<Option<EmployeeModel>, Error> {
                let employee = {
                        let mut repo = self.unit_of_work.employee_repo();
                        if is_soft {
                                repo.soft_delete(id).await?
                        }
                        else {
                                repo.delete(id).await?
                        }
                };

                self.unit_of_work.commit().await?;

                Ok(employee)
        }

        async fn restore(mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, Error> {
                let employee = {
                        let mut repo = self.unit_of_work.employee_repo();
                        repo.restore(id).await?
                };

                self.unit_of_work.commit().await?;

                Ok(employee)
        }
}