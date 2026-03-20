use async_trait::async_trait;
use crate::employee::EmployeeModel;
use crate::employee::salary::Salary;
use conservatory_core::id::Id;

#[async_trait]
pub trait EmployeeService: Send + Sync {
        async fn create(&self, employee: &EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(&self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn list(&self) -> Result<Vec<EmployeeModel>, anyhow::Error>;
        async fn update_salary(&self, id: &Id, salary: &Salary) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn delete(&self, id: &Id, is_soft: bool) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn restore(&self, id: &Id) -> Result<Option<EmployeeModel>, anyhow::Error>;
}