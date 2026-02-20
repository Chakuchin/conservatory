use crate::employee::EmployeeModel;
use crate::employee::id::EmployeeId;
use crate::employee::salary::Salary;

#[allow(async_fn_in_trait)]
pub trait EmployeeRepository {
        async fn create(&mut self, employee: EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<EmployeeModel>, anyhow::Error>;
        async fn update_salary(&mut self, id: EmployeeId, salary: Salary) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn soft_delete(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn delete(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn restore(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
}