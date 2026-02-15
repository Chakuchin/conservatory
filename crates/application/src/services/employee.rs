use conservatory_model::di::unit_of_work::UnitOfWork;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::identities::employee_id::EmployeeId;
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
}