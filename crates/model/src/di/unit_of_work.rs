use async_trait::async_trait;
use crate::repositories::sql::employee::EmployeeRepository;
use crate::repositories::sql::greenhouse::GreenhouseRepository;
use crate::repositories::sql::plant::PlantRepository;

#[async_trait]
pub trait UnitOfWork<'a>: Send + Sync {
        type EmployeeRepo<'b>: EmployeeRepository where Self: 'b;
        type GreenhouseRepo<'b>: GreenhouseRepository where Self: 'b;
        type PlantRepo<'b>: PlantRepository where Self: 'b;

        fn employee_repo(&mut self) -> Self::EmployeeRepo<'_>;
        fn greenhouse_repo(&mut self) -> Self::GreenhouseRepo<'_>;
        fn plant_repo(&mut self) -> Self::PlantRepo<'_>;

        async fn commit(self) -> Result<(), anyhow::Error>;
        async fn rollback(self) -> Result<(), anyhow::Error>;
}