use crate::repositories::employee::EmployeeRepository;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork {
        type EmployeeRepo<'b>: EmployeeRepository where Self: 'b;

        fn employee_repo(&mut self) -> Self::EmployeeRepo<'_>;

        async fn commit(self) -> Result<(), anyhow::Error>;
}