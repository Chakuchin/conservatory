use actix_web::{delete, get, post, HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use conservatory_model::services::employee::EmployeeService;
use crate::inbound::employee::dto::EmployeeDTO;
use crate::inbound::employee::dto::path::EmployeeIdDTO;

#[utoipa::path(params(EmployeeIdDTO))]
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

#[utoipa::path]
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

#[utoipa::path]
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

#[utoipa::path(params(EmployeeIdDTO))]
#[delete("/{employee_id}")]
pub async fn delete_employee(service: Data<dyn EmployeeService>, id: Path<EmployeeIdDTO>) -> HttpResponse {
        let res = service.delete(&id, true).await;

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