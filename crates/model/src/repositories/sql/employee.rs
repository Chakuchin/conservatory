use async_trait::async_trait;
use crate::employee::EmployeeModel;
use crate::employee::salary::Salary;
use conservatory_core::id::Id;

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
        async fn create(&mut self, employee: &EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(&mut self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<EmployeeModel>, anyhow::Error>;
        async fn update_salary(&mut self, id: &Id, salary: &Salary) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn soft_delete(&mut self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn delete(&mut self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn restore(&mut self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
}