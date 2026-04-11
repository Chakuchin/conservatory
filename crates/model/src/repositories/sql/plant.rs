use async_trait::async_trait;
use crate::plant::plant_type::PlantTypeModel;
use conservatory_core::id::{Id, TypeId};
use crate::employee::employee_plant_work::EmployeePlantWorkModel;
use crate::enums::WorkType;
use crate::plant::PlantModel;

#[async_trait]
pub trait PlantRepository: Send + Sync {
        async fn register(&mut self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, anyhow::Error>;
        async fn create(&mut self, id: &Id, type_urn: &TypeId) -> Result<PlantModel, anyhow::Error>;
        async fn get_type(&mut self, type_urn: &TypeId) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn get(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
        async fn list_types(&mut self) -> Result<Vec<PlantTypeModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<PlantModel>, anyhow::Error>;
        async fn update_type_description(&mut self, type_urn: &TypeId, description: &str) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn soft_delete_plant(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
        async fn delete_plant(&mut self, plant_id: &Id) -> Result<Option<()>, anyhow::Error>;
        async fn delete_type(&mut self, type_urn: &TypeId) -> Result<Option<()>, anyhow::Error>;
        async fn restore_plant(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;

        async fn work_with(&mut self, plant_id: &Id, employee_id: &Id, work_type: &WorkType) -> Result<Option<EmployeePlantWorkModel>, anyhow::Error>;
        async fn plant_at(&mut self, plant_id: &Id, greenhouse_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
        async fn remove_from(&mut self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
}