use async_trait::async_trait;
use uuid::fmt::Urn;
use crate::plant_type::PlantTypeModel;

#[async_trait]
pub trait EmployeeService: Send + Sync {
        async fn create(&self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, anyhow::Error>;
        async fn get(&self, id: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn list(&self) -> Result<Vec<PlantTypeModel>, anyhow::Error>;
        async fn update_description(&self, id: &Urn, salary: &str) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn delete(&self, id: &Urn, is_soft: bool) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn restore(&self, id: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
}