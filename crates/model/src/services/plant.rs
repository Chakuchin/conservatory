use async_trait::async_trait;
use conservatory_core::id::{Id, TypeId};
use crate::employee::employee_plant_work::EmployeePlantWorkModel;
use crate::enums::WorkType;
use crate::plant::plant_type::PlantTypeModel;
use crate::plant::PlantModel;

#[async_trait]
pub trait PlantService: Send + Sync {
        async fn register(&self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, anyhow::Error>;
        async fn create(&self, id: &Id, type_urn: &TypeId) -> Result<PlantModel, anyhow::Error>;
        async fn get_type(&self, type_urn: &TypeId) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn get(&self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
        async fn list_types(&self) -> Result<Vec<PlantTypeModel>, anyhow::Error>;
        async fn list(&self) -> Result<Vec<PlantModel>, anyhow::Error>;
        async fn update_type_description(&self, type_urn: &TypeId, description: &str) -> Result<Option<PlantTypeModel>, anyhow::Error>;
        async fn delete_plant(&self, plant_id: &Id, is_soft: bool) -> Result<Option<()>, anyhow::Error>;
        async fn delete_type(&self, type_urn: &TypeId) -> Result<Option<()>, anyhow::Error>;
        async fn restore_plant(&self, plant_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;

        async fn work_with(&self, employee_id: &Id, plant_id: &Id, work_type: &WorkType) -> Result<Option<EmployeePlantWorkModel>, anyhow::Error>;
        async fn plant_at(&self, plant_id: &Id, employee_id: &Id, greenhouse_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;
        async fn remove_from(&self, plant_id: &Id, employee_id: &Id) -> Result<Option<PlantModel>, anyhow::Error>;        
}