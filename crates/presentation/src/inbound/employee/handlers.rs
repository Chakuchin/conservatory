use actix_web::{delete, get, patch, post, HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query};
use conservatory_model::services::employee::EmployeeService;
use crate::inbound::common::dto::query::IsSoftDTO;
use crate::inbound::employee::dto::body::SalaryDTO;
use crate::inbound::employee::dto::EmployeeDTO;
use crate::inbound::employee::dto::path::EmployeeIdDTO;
use crate::open_api::EMPLOYEE_TAG;

#[utoipa::path(tag = EMPLOYEE_TAG)]
#[get("/{employee_id}")]
pub async fn get_employee(service: Data<dyn EmployeeService>, id: Path<EmployeeIdDTO>) -> HttpResponse {
        let res = service.get(&id).await;

        match res {
                Ok(employee) => {
                        match employee {
                                Some(employee) => HttpResponseBuilder::new(StatusCode::OK).json(EmployeeDTO::from(employee)),
                                None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                        }
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}

#[utoipa::path(tag = EMPLOYEE_TAG)]
#[post("/")]
pub async fn create_employee(service: Data<dyn EmployeeService>, employee: Json<EmployeeDTO>) -> HttpResponse {
        let res = service.create(&employee).await;

        match res {
                Ok(employee) => {
                        HttpResponseBuilder::new(StatusCode::CREATED).json(EmployeeDTO::from(employee))
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}

#[utoipa::path(tag = EMPLOYEE_TAG)]
#[get("/")]
pub async fn list_employees(service: Data<dyn EmployeeService>) -> HttpResponse {
        let res = service.list().await;

        match res {
                Ok(employees) => {
                        HttpResponseBuilder::new(StatusCode::OK)
                                .json(employees.into_iter().map(EmployeeDTO::from).collect::<Vec<EmployeeDTO>>())
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(EmployeeIdDTO, IsSoftDTO))]
#[delete("/{employee_id}")]
pub async fn delete_employee(service: Data<dyn EmployeeService>, id: Path<EmployeeIdDTO>, is_soft: Query<IsSoftDTO>) -> HttpResponse {
        let res = service.delete(&id, **is_soft).await;

        match res {
                Ok(employee) => {
                        match employee {
                                Some(employee) => HttpResponseBuilder::new(StatusCode::NO_CONTENT).json(EmployeeDTO::from(employee)),
                                None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                        }
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(EmployeeIdDTO))]
#[patch("/{employee_id}")]
pub async fn update_employee_salary(service: Data<dyn EmployeeService>, id: Path<EmployeeIdDTO>, salary: Json<SalaryDTO>) -> HttpResponse {
        let res = service.update_salary(&id, &salary).await;

        match res {
                Ok(employee) => {
                        match employee {
                                Some(employee) => HttpResponseBuilder::new(StatusCode::OK).json(EmployeeDTO::from(employee)),
                                None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                        }
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}

#[utoipa::path(tag = EMPLOYEE_TAG, params(EmployeeIdDTO))]
#[post("/{employee_id}/restore")]
pub async fn restore_employee(service: Data<dyn EmployeeService>, id: Path<EmployeeIdDTO>) -> HttpResponse {
        let res = service.restore(&id).await;

        match res {
                Ok(employee) => {
                        match employee {
                                Some(employee) => HttpResponseBuilder::new(StatusCode::OK).json(EmployeeDTO::from(employee)),
                                None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                        }
                }
                Err(err) => {
                        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err.to_string())
                }
        }
}