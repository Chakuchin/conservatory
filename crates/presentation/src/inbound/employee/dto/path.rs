use std::ops::Deref;
use conservatory_model::employee::id::EmployeeId;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(names("employee_id"), parameter_in = Path)]
pub struct EmployeeIdDTO(#[param(value_type = Uuid)] EmployeeId);

impl Deref for EmployeeIdDTO {
        type Target = EmployeeId;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}