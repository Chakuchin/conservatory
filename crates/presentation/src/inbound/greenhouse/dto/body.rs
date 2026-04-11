use std::ops::Deref;
use serde::Deserialize;
use utoipa::{PartialSchema, ToSchema};
use utoipa::openapi::{RefOr, Schema};
use uuid::Uuid;
use conservatory_model::enums::Condition;

#[derive(Debug, Deserialize)]
pub struct ConditionDTO(Condition);

impl From<Condition> for ConditionDTO {
        fn from(value: Condition) -> Self {
                Self(value)
        }
}

impl Deref for ConditionDTO {
        type Target = Condition;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for ConditionDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a>(&'a str);

                SchemaHint::schema()
        }
}

impl ToSchema for ConditionDTO {}