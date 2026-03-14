use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::Currency;

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