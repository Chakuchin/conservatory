pub mod swagger;

use utoipa::OpenApi;
use crate::inbound::employee::dto::body::SalaryDTO;

pub const EMPLOYEE_TAG: &str = "employee";

#[derive(Debug, OpenApi)]
#[openapi(
        tags((name = EMPLOYEE_TAG)),
        components(schemas(SalaryDTO))
)]
pub struct ApiDoc;