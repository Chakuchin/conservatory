use std::ops::Deref;
use actix_web::{delete, get, patch, post, HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query};
use conservatory_model::services::employee::EmployeeService;
use conservatory_model::services::greenhouse::GreenhouseService;
use conservatory_model::services::plant::PlantService;
use crate::inbound::common::dto::path::{IdDTO, TypeIdDTO};
use crate::inbound::common::dto::query::IsSoftDTO;
use crate::inbound::common::error::{ConservatoryOptionExt, ConservatoryResultExt};
use crate::inbound::employee::dto::body::{EmployeePlantWorkDTO, SalaryDTO};
use crate::inbound::employee::dto::EmployeeDTO;
use crate::inbound::greenhouse::dto::GreenhouseDTO;
use crate::inbound::plant::dto::body::{CreatePlantDTO, EmployeeGreenhouseDTO, PlantTypeDTO};
use crate::inbound::plant::dto::PlantDTO;
use crate::open_api::PLANT_TAG;

#[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
#[get("/{id}")]
pub async fn get_plant(service: Data<dyn PlantService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.get(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| plant.map(PlantDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG, params(TypeIdDTO))]
#[get("/{id}")]
pub async fn get_plant_type(service: Data<dyn PlantService>, id: Path<TypeIdDTO>) -> HttpResponse {
        let res = service.get_type(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant_type| plant_type.map(PlantTypeDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG)]
#[post("")]
pub async fn create_plant(service: Data<dyn PlantService>, plant: Json<CreatePlantDTO>) -> HttpResponse {
        let res =  service.create(&plant.id, &plant.type_urn).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| HttpResponseBuilder::new(StatusCode::CREATED).json(PlantDTO::from(plant))
                )
}

#[utoipa::path(tag = PLANT_TAG)]
#[post("")]
pub async fn register_plant_type(service: Data<dyn PlantService>, plant_type: Json<PlantTypeDTO>) -> HttpResponse {
        let res =  service.register(&plant_type).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant_type| HttpResponseBuilder::new(StatusCode::CREATED).json(PlantTypeDTO::from(plant_type))
                )
}

#[utoipa::path(tag = PLANT_TAG)]
#[get("")]
pub async fn list_plants(service: Data<dyn PlantService>) -> HttpResponse {
        let res = service.list().await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| {
                                HttpResponseBuilder::new(StatusCode::OK)
                                        .json(plant.into_iter().map(PlantDTO::from).collect::<Vec<_>>())
                        }
                )
}

#[utoipa::path(tag = PLANT_TAG)]
#[get("")]
pub async fn list_plant_types(service: Data<dyn PlantService>) -> HttpResponse {
        let res = service.list_types().await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant_type| {
                                HttpResponseBuilder::new(StatusCode::OK)
                                        .json(plant_type.into_iter().map(PlantTypeDTO::from).collect::<Vec<_>>())
                        }
                )
}

#[utoipa::path(tag = PLANT_TAG, params(IdDTO, IsSoftDTO))]
#[delete("/{id}")]
pub async fn delete_plant(service: Data<dyn PlantService>, id: Path<IdDTO>, is_soft: Query<IsSoftDTO>) -> HttpResponse {
        let res = service.delete_plant(&id, **is_soft).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| plant.no_content_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
#[delete("/{id}")]
pub async fn delete_plant_type(service: Data<dyn PlantService>, id: Path<TypeIdDTO>) -> HttpResponse {
        let res = service.delete_type(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant_type| plant_type.no_content_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
#[patch("/{id}/plant")]
pub async fn plant_at(service: Data<dyn PlantService>, id: Path<IdDTO>, employee_greenhouse_id: Json<EmployeeGreenhouseDTO>) -> HttpResponse {
        let res = service.plant_at(&id, &employee_greenhouse_id.employee_id, &employee_greenhouse_id.greenhouse_id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| plant.map(PlantDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
#[patch("/{id}/remove")]
pub async fn remove_from(service: Data<dyn PlantService>, id: Path<IdDTO>, employee_id: Json<IdDTO>) -> HttpResponse {
        let res = service.remove_from(&id, &employee_id.0).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| plant.map(PlantDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
#[post("/{id}/restore")]
pub async fn restore_plant(service: Data<dyn PlantService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.restore_plant(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |plant| plant.map(PlantDTO::from).ok_or_not_found()
                )
}

// #[utoipa::path(tag = PLANT_TAG, params(IdDTO))]
// #[patch("/{id}/work")]
// pub async fn work_with(service: Data<dyn PlantService>, id: Path<IdDTO>, employee_plant_work: Json<EmployeePlantWorkDTO>) -> HttpResponse {
//         let res = service.work_with(&id, &employee_plant_work.employee_id, &employee_plant_work.work_type).await;
//
//         res.map_err(|err| err.to_string())
//                 .map_or_internal_server_error(
//                         |plant| plant.map(EmployeePlantWorkReturnDTO::from).ok_or_not_found()
//                 )
// }