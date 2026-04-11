pub mod swagger;

use utoipa::OpenApi;
use crate::inbound::employee::dto::body::SalaryDTO;

pub const EMPLOYEE_TAG: &'static str = "employee";
pub const GREENHOUSE_TAG: &'static str = "greenhouse";
pub const PLANT_TAG: &'static str = "plant";

#[derive(Debug, OpenApi)]
#[openapi(
        tags((name = EMPLOYEE_TAG), (name = GREENHOUSE_TAG), (name = PLANT_TAG)),
        components(schemas(SalaryDTO))
)]
pub struct ApiDoc;