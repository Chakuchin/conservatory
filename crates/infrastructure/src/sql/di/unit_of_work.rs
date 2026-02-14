use sqlx::PgTransaction;
use conservatory_model::di::unit_of_work::UnitOfWork;
use crate::sql::repositories::employee::EmployeePostgresqlRepository;

#[derive(Debug)]
pub struct PgUnitOfWork<'a> {
        pub tx: PgTransaction<'a>
}

impl<'a> UnitOfWork<'a> for PgUnitOfWork<'a> {
        type EmployeeRepo<'b> = EmployeePostgresqlRepository<'b, 'a> where Self: 'b, Self: 'a;

        fn employee_repo<'b>(&'b mut self) -> Self::EmployeeRepo<'b> {
                Self::EmployeeRepo::new(&mut self.tx)
        }

        async fn commit(self) -> Result<(), anyhow::Error> {
                self.tx.commit().await?;
                Ok(())
        }
}