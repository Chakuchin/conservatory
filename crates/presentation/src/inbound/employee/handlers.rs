use actix_web::{delete, get, patch, post, HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query};
use conservatory_model::services::employee::EmployeeService;
use crate::inbound::common::dto::query::IsSoftDTO;
use crate::inbound::common::dto::path::IdDTO;
use crate::inbound::employee::dto::body::{EmployeePlantWorkDTO, SalaryDTO, WorkTypeDTO};
use crate::inbound::employee::dto::EmployeeDTO;
use crate::open_api::EMPLOYEE_TAG;
use crate::inbound::common::error::{ConservatoryOptionExt, ConservatoryResultExt};

#[utoipa::path(tag = EMPLOYEE_TAG, params(IdDTO))]
#[get("/{id}")]
pub async fn get_employee(service: Data<dyn EmployeeService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.get(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employee| employee.map(EmployeeDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = EMPLOYEE_TAG)]
#[post("")]
pub async fn create_employee(service: Data<dyn EmployeeService>, employee: Json<EmployeeDTO>) -> HttpResponse {
        let res = service.create(&employee).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employee| HttpResponseBuilder::new(StatusCode::CREATED).json(EmployeeDTO::from(employee))
                )
}

#[utoipa::path(tag = EMPLOYEE_TAG)]
#[get("")]
pub async fn list_employees(service: Data<dyn EmployeeService>) -> HttpResponse {
        let res = service.list().await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employees| {
                                HttpResponseBuilder::new(StatusCode::OK)
                                        .json(employees.into_iter().map(EmployeeDTO::from).collect::<Vec<EmployeeDTO>>())
                        }
                )
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(IdDTO, IsSoftDTO))]
#[delete("/{id}")]
pub async fn delete_employee(service: Data<dyn EmployeeService>, id: Path<IdDTO>, is_soft: Query<IsSoftDTO>) -> HttpResponse {
        let res = service.delete(&id, **is_soft).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employee| employee.map(EmployeeDTO::from).no_content_or_not_found()
                )
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(IdDTO))]
#[patch("/{id}")]
pub async fn update_employee_salary(service: Data<dyn EmployeeService>, id: Path<IdDTO>, salary: Json<SalaryDTO>) -> HttpResponse {
        let res = service.update_salary(&id, &salary).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employee| employee.map(EmployeeDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(IdDTO))]
#[post("/{id}/restore")]
pub async fn restore_employee(service: Data<dyn EmployeeService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.restore(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |employee| employee.map(EmployeeDTO::from).ok_or_not_found()
                )
}
