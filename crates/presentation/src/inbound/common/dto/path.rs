use std::ops::Deref;
use conservatory_core::id::Id;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("id"), parameter_in = Path)]
pub struct IdDTO(#[param(value_type = Uuid)] Id);

impl Deref for IdDTO {
        type Target = Id;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}