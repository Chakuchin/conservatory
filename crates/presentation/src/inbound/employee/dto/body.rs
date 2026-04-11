use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::{Currency, WorkType};

#[derive(Debug)]
pub struct SalaryDTO(Salary);

impl Deref for SalaryDTO {
        type Target = Salary;
        
        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl From<Salary> for SalaryDTO {
        fn from(value: Salary) -> Self { Self(value) }
}

impl PartialSchema for SalaryDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        amount: &'a u32,
                        #[schema(example = "RUB")]
                        currency: &'a String,
                }

                SchemaHint::schema()
        }
}

impl ToSchema for SalaryDTO {}

impl Serialize for SalaryDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        amount: &'a u32,
                        currency: &'a Currency
                }

                let hint = SerializeHint {
                        amount: &self.0.amount,
                        currency: &self.0.currency
                };

                hint.serialize(serializer)
        }
}

impl<'de> Deserialize<'de> for SalaryDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        amount: u32,
                        currency: Currency
                }

                let helper = DeserializeHint::deserialize(deserializer)?;
                Ok(SalaryDTO(Salary::new(helper.amount, helper.currency)))
        }
}

#[derive(Debug)]
pub struct WorkTypeDTO(WorkType);

impl Deref for WorkTypeDTO {
        type Target = WorkType;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl From<WorkType> for WorkTypeDTO {
        fn from(value: WorkType) -> Self { Self(value) }
}

impl PartialSchema for WorkTypeDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        #[schema(example = "CULTIVATE")]
                        work_type: &'a str,
                }

                SchemaHint::schema()
        }
}

impl ToSchema for WorkTypeDTO {}

impl<'de> Deserialize<'de> for WorkTypeDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        work_type: WorkType,
                }

                let helper = DeserializeHint::deserialize(deserializer)?;
                Ok(Self(helper.work_type))
        }
}

#[derive(Debug)]
pub struct EmployeePlantWorkDTO(EmployeePlantWorkModel);

impl Deref for EmployeePlantWorkDTO {
        type Target = EmployeePlantWorkModel;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl From<EmployeePlantWorkModel> for EmployeePlantWorkDTO {
        fn from(value: EmployeePlantWorkModel) -> Self { Self(value) }
}

impl PartialSchema for EmployeePlantWorkDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        employee_id: &'a Uuid,
                        plant_id: &'a Uuid,
                        #[schema(example = "CULTIVATE")]
                        work_type: &'a str,
                }

                SchemaHint::schema()
        }
}

impl ToSchema for EmployeePlantWorkDTO {}

impl Serialize for EmployeePlantWorkDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        employee_id: &'a Uuid,
                        plant_id: &'a Uuid,
                        work_type: &'a WorkType,
                }

                let hint = SerializeHint {
                        employee_id: &self.0.employee_id,
                        plant_id: &self.0.plant_id,
                        work_type: &self.0.work_type
                };

                hint.serialize(serializer)
        }
}