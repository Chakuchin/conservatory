use crate::employee::EmployeeModel;
use crate::employee::id::EmployeeId;
use crate::employee::salary::Salary;

#[allow(async_fn_in_trait)]
pub trait EmployeeService {
        async fn create(self, employee: EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn list(self) -> Result<Vec<EmployeeModel>, anyhow::Error>;
        async fn update_salary(self, id: EmployeeId, salary: Salary) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn delete(self, id: EmployeeId, is_soft: bool) -> Result<Option<EmployeeModel>, anyhow::Error>;
        async fn restore(self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
}