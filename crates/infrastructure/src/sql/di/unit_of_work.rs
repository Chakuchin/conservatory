use async_trait::async_trait;
use sqlx::PgTransaction;
use conservatory_model::di::unit_of_work::UnitOfWork;
use crate::sql::repositories::employee::EmployeePostgresqlRepository;

#[derive(Debug)]
pub struct PgUnitOfWork<'a> {
        pub tx: PgTransaction<'a>
}

#[async_trait]
impl<'a> UnitOfWork<'a> for PgUnitOfWork<'a> {
        type EmployeeRepo<'b> = EmployeePostgresqlRepository<'b, 'a> where Self: 'b;

        fn employee_repo(&mut self) -> Self::EmployeeRepo<'_> {
                Self::EmployeeRepo::new(&mut self.tx)
        }

        async fn commit(self) -> Result<(), anyhow::Error> {
                self.tx.commit().await?;
                Ok(())
        }
        
        async fn rollback(self) -> Result<(), anyhow::Error> {
                self.tx.rollback().await?;
                Ok(())
        }
}