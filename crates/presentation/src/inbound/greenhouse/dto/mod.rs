pub mod body;

use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use conservatory_model::greenhouse::{GreenhouseModel, SquareMeters};
use crate::inbound::greenhouse::dto::body::ConditionDTO;

#[derive(Debug)]
pub struct GreenhouseDTO(GreenhouseModel);

impl From<GreenhouseModel> for GreenhouseDTO {
        fn from(value: GreenhouseModel) -> Self {
                Self(value)
        }
}

impl Deref for GreenhouseDTO {
        type Target = GreenhouseModel;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for GreenhouseDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        #[schema(read_only)]
                        id: &'a Uuid,
                        name: &'a str,
                        humidity: &'a u32,
                        target_temperature: &'a f32,
                        area: &'a f32,
                        #[schema(read_only, ignore)]
                        conditions: Vec<ConditionDTO>,
                }

                SchemaHint::schema()
        }
}

impl ToSchema for GreenhouseDTO {}

impl Serialize for GreenhouseDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        id: &'a Id,
                        name: &'a str,
                        humidity: &'a u32,
                        target_temperature: &'a f32,
                        area: &'a f32,
                        conditions: Vec<String>
                }

                let hint = SerializeHint {
                        id: &self.id,
                        name: &self.name,
                        humidity: &(self.humidity as u32),
                        target_temperature: &self.target_temperature.celsius(),
                        area: &self.area.to_num::<f32>(),
                        conditions: (&self.conditions).into_iter().map(|c| c.to_string()).collect()
                };

                hint.serialize(serializer)
        }
}

impl<'de> Deserialize<'de> for GreenhouseDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        name: String,
                        humidity: RelativeHumidity,
                        target_temperature: f32,
                        area: SquareMeters
                }

                let helper = DeserializeHint::deserialize(deserializer)?;
                Ok(Self(GreenhouseModel::new(
                        Id::new(),
                        &helper.name,
                        helper.humidity,
                        Temperature::<Celsius>::new(helper.target_temperature),
                        &[],
                        helper.area,
                )))
        }
}