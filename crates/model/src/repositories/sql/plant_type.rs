use async_trait::async_trait;
use crate::plant_type::PlantTypeModel;
use uuid::fmt::Urn;

#[async_trait]
pub trait PlantTypeRepository: Send + Sync {
        async fn create(&mut self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, anyhow::Error>;
        async fn get(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<PlantTypeModel>, anyhow::Error>;
        async fn update_description(&mut self, urn: &Urn, description: &str) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn soft_delete(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn delete(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn restore(&mut self, urn: &Urn) -> Result<Option<PlantTypeModel>, anyhow::Error>;
}