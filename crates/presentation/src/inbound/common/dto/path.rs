use std::ops::Deref;
use conservatory_core::id::{Id, TypeId};
use serde::Deserialize;
use utoipa::{IntoParams, PartialSchema, ToSchema};
use utoipa::openapi::{RefOr, Schema};
use uuid::Uuid;
use uuid::fmt::Urn;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("id"), parameter_in = Path)]
pub struct IdDTO(#[param(value_type = Uuid)] Id);

impl Deref for IdDTO {
        type Target = Id;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for IdDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a>(&'a Uuid);

                SchemaHint::schema()
        }
}

impl ToSchema for IdDTO {}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("id"), parameter_in = Path)]
pub struct TypeIdDTO(#[param(value_type = Uuid)] TypeId);

impl Deref for TypeIdDTO {
        type Target = TypeId;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl PartialSchema for TypeIdDTO {
        fn schema() -> RefOr<Schema> {
                #[derive(ToSchema)]
                struct SchemaHint<'a>(&'a String);

                SchemaHint::schema()
        }
}

impl ToSchema for TypeIdDTO {}