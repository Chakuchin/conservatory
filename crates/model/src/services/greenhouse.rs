use async_trait::async_trait;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use crate::enums::Condition;
use crate::greenhouse::GreenhouseModel;

#[async_trait]
pub trait GreenhouseService: Send + Sync {
        async fn create(&self, greenhouse: &GreenhouseModel) -> Result<GreenhouseModel, anyhow::Error>;
        async fn get(&self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn list(&self) -> Result<Vec<GreenhouseModel>, anyhow::Error>;
        async fn update_humidity(&self, id: &Id, humidity: RelativeHumidity) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn update_target_temperature(&self, id: &Id, temperature: Temperature<Celsius>) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn delete(&self, id: &Id, is_soft: bool) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn restore(&self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        
        async fn add_condition(&self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn remove_condition(&self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, anyhow::Error>;
}