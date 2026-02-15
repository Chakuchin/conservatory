use crate::employee::EmployeeModel;
use crate::identities::employee_id::EmployeeId;

#[allow(async_fn_in_trait)]
pub trait EmployeeService {
        async fn create(self, employee: EmployeeModel) -> Result<EmployeeModel, anyhow::Error>;
        async fn get(self, id: EmployeeId) -> Result<Option<EmployeeModel>, anyhow::Error>;
}