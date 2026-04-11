use async_trait::async_trait;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use crate::enums::Condition;
use crate::greenhouse::GreenhouseModel;

#[async_trait]
pub trait GreenhouseRepository: Send + Sync {
        async fn create(&mut self, greenhouse: &GreenhouseModel) -> Result<GreenhouseModel, anyhow::Error>;
        async fn get(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn list(&mut self) -> Result<Vec<GreenhouseModel>, anyhow::Error>;
        async fn update_humidity(&mut self, id: &Id, humidity: RelativeHumidity) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn update_target_temperature(&mut self, id: &Id, temperature: Temperature<Celsius>) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn soft_delete(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn delete(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn restore(&mut self, id: &Id) -> Result<Option<GreenhouseModel>, anyhow::Error>;

        async fn add_condition(&mut self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, anyhow::Error>;
        async fn remove_condition(&mut self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, anyhow::Error>;
}