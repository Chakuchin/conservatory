use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;
use conservatory_model::plant::PlantModel;
use crate::inbound::common::dto::path::IdDTO;
use crate::inbound::plant::dto::body::PlantTypeDTO;

pub mod body;

#[derive(Debug)]
pub struct PlantDTO(PlantModel);

impl From<PlantModel> for PlantDTO {
        fn from(value: PlantModel) -> Self {
                Self(value)
        }
}

impl Deref for PlantDTO {
        type Target = PlantModel;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for PlantDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        #[schema(read_only)]
                        id: &'a Uuid,
                        plant_type: &'a PlantTypeDTO,
                        greenhouse_id: Option<&'a Uuid>
                }

                SchemaHint::schema()
        }
}

impl ToSchema for PlantDTO {}

impl Serialize for PlantDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        id: &'a Uuid,
                        plant_type: PlantTypeDTO,
                        greenhouse_id: Option<&'a Uuid>
                }

                let hint = SerializeHint {
                        id: self.id.deref(),
                        plant_type: self.plant_type.clone().into(),
                        greenhouse_id: self.greenhouse_id.as_ref().map(|inner| inner.deref())
                };

                hint.serialize(serializer)
        }
}