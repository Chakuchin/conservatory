use async_trait::async_trait;
use crate::repositories::sql::employee::EmployeeRepository;

#[async_trait]
pub trait UnitOfWork<'a>: Send + Sync {
        type EmployeeRepo<'b>: EmployeeRepository where Self: 'b;

        fn employee_repo(&mut self) -> Self::EmployeeRepo<'_>;

        async fn commit(self) -> Result<(), anyhow::Error>;
        async fn rollback(self) -> Result<(), anyhow::Error>;
}