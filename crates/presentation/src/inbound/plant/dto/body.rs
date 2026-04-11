use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::fmt::Urn;
use uuid::Uuid;
use conservatory_core::id::{Id, TypeId};
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::enums::WorkType;
use conservatory_model::plant::PlantTypeModel;
use crate::inbound::common::dto::path::IdDTO;

#[derive(Debug)]
pub struct PlantTypeDTO(PlantTypeModel);

impl From<PlantTypeModel> for PlantTypeDTO {
        fn from(value: PlantTypeModel) -> Self {
                Self(value)
        }
}

impl Deref for PlantTypeDTO {
        type Target = PlantTypeModel;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for PlantTypeDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        urn: &'a Uuid,
                        name: &'a str,
                        description: &'a str
                }

                SchemaHint::schema()
        }
}

impl ToSchema for PlantTypeDTO {}

impl Serialize for PlantTypeDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        urn: &'a Uuid,
                        name: &'a str,
                        description: &'a str
                }

                let hint = SerializeHint {
                        urn: self.urn.as_uuid(),
                        name: &self.name,
                        description: &self.description
                };

                hint.serialize(serializer)
        }
}

impl<'de> Deserialize<'de> for PlantTypeDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        urn: Urn,
                        name: String,
                        description: String
                }

                let helper = DeserializeHint::deserialize(deserializer)?;

                Ok(Self(PlantTypeModel::new(
                        helper.urn,
                        helper.name,
                        helper.description
                )))
        }
}

#[derive(Debug)]
pub struct CreatePlantDTO {
        pub id: Id,
        pub type_urn: TypeId
}

impl PartialSchema for CreatePlantDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        #[schema(read_only)]
                        id: &'a Uuid,
                        type_urn: &'a Uuid
                }

                SchemaHint::schema()
        }
}

impl ToSchema for CreatePlantDTO {}

impl<'de> Deserialize<'de> for CreatePlantDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        type_urn: Urn
                }

                let helper = DeserializeHint::deserialize(deserializer)?;

                Ok(Self {
                        id: Id::new(),
                        type_urn: helper.type_urn.into()
                })
        }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmployeeGreenhouseDTO {
        pub employee_id: IdDTO,
        pub greenhouse_id: IdDTO
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmployeePlantWorkDTO {
        pub employee_id: IdDTO,
        #[schema(value_type = String)]
        pub work_type: WorkType
}

// #[derive(Debug, Serialize)]
// pub struct EmployeePlantWorkReturnDTO {
//         pub id: IdDTO,
//         pub employee_id: IdDTO,
//         pub plant_id: IdDTO,
//         pub work_type: String
// }

// impl From<EmployeePlantWorkModel> for EmployeePlantWorkReturnDTO {
//         fn from(value: EmployeePlantWorkModel) -> Self {
//                 Self {
//                         id: value.id.into(),
//                         employee_id: value.employee_id.into(),
//                         plant_id: value.plant_id.into(),
//                         work_type: value.work_type.to_string()
//                 }
//         }
// }