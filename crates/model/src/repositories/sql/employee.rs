use async_trait::async_trait;
use crate::employee::EmployeeModel;
use crate::employee::id::EmployeeId;
use crate::employee::salary::Salary;

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
        async fn create(&mut self, employee: &EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(&mut self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<EmployeeModel>, anyhow::Error>;
        async fn update_salary(&mut self, id: &EmployeeId, salary: &Salary) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn soft_delete(&mut self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn delete(&mut self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn restore(&mut self, id: &EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
}