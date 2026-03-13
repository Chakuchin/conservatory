use std::ops::Deref;
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use time::Date;
use utoipa::{PartialSchema, ToSchema};
use utoipa::openapi::{RefOr, Schema};
use uuid::Uuid;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::id::EmployeeId;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::Currency;
use conservatory_core::date::core_date;

pub mod path;

#[derive(Debug)]
pub struct EmployeeDTO(EmployeeModel);

impl From<EmployeeModel> for EmployeeDTO {
        fn from(value: EmployeeModel) -> Self {
                Self(value)
        }
}

impl Deref for EmployeeDTO {
        type Target = EmployeeModel;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for EmployeeDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a> {
                        #[schema(read_only)]
                        id: &'a Uuid,
                        name: &'a str,
                        surname: &'a str,
                        patronymic: Option<&'a str>,
                        amount: &'a u32,
                        #[schema(example = "RUB")]
                        currency: &'a String,
                        works_since: &'a Date,
                }

                SchemaHint::schema()
        }
}

impl ToSchema for EmployeeDTO {}

impl Serialize for EmployeeDTO {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: Serializer,
        {
                #[derive(Serialize)]
                struct SerializeHint<'a> {
                        id: &'a EmployeeId,
                        name: &'a str,
                        surname: &'a str,
                        patronymic: Option<&'a str>,
                        amount: &'a u32,
                        currency: &'a Currency,
                        #[serde(with = "core_date")]
                        works_since: &'a Date,
                }

                let hint = SerializeHint {
                        id: &self.0.id,
                        name: &self.0.name,
                        surname: &self.0.surname,
                        patronymic: self.0.patronymic.as_deref(),
                        amount: &self.0.salary.amount,
                        currency: &self.0.salary.currency,
                        works_since: &self.0.works_since
                };

                hint.serialize(serializer)
        }
}

impl<'de> Deserialize<'de> for EmployeeDTO {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: Deserializer<'de>,
        {
                #[derive(Deserialize)]
                struct DeserializeHint {
                        name: String,
                        surname: String,
                        patronymic: Option<String>,
                        amount: u32,
                        currency: Currency,
                        #[serde(with = "core_date")]
                        works_since: Date,
                }

                let helper = DeserializeHint::deserialize(deserializer)?;
                Ok(EmployeeDTO(EmployeeModel::new(
                        helper.name,
                        helper.surname,
                        helper.patronymic,
                        Salary::new(helper.amount, helper.currency),
                        helper.works_since,
                )))
        }
}