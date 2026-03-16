use std::ops::Deref;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IsSoftDTO {
        #[serde(rename = "is_soft")]
        is_soft: bool
}

impl Deref for IsSoftDTO {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
                &self.is_soft
        }
}
