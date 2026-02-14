use crate::employee::EmployeeModel;
use crate::identities::employee_id::EmployeeId;

#[allow(async_fn_in_trait)]
pub trait EmployeeRepository {
        async fn create(&mut self, employee: EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(&mut self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
}