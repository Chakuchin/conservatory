use crate::repositories::employee::EmployeeRepository;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<'a> {
        type EmployeeRepo<'b>: EmployeeRepository where Self: 'b, Self: 'a;

        fn employee_repo<'b>(&'b mut self) -> Self::EmployeeRepo<'b>;

        async fn commit(self) -> Result<(), anyhow::Error>;
}