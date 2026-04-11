use std::ops::Deref;
use actix_web::{delete, get, patch, post, HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query};
use conservatory_model::enums::Condition;
use conservatory_model::services::greenhouse::GreenhouseService;
use crate::inbound::common::dto::path::IdDTO;
use crate::inbound::common::dto::query::IsSoftDTO;
use crate::inbound::common::error::{ConservatoryOptionExt, ConservatoryResultExt};
use crate::inbound::greenhouse::dto::body::ConditionDTO;
use crate::inbound::greenhouse::dto::GreenhouseDTO;
use crate::open_api::GREENHOUSE_TAG;

#[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO))]
#[get("/{id}")]
pub async fn get_greenhouse(service: Data<dyn GreenhouseService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.get(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| greenhouse.map(GreenhouseDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = GREENHOUSE_TAG)]
#[post("")]
pub async fn create_greenhouse(service: Data<dyn GreenhouseService>, greenhouse: Json<GreenhouseDTO>) -> HttpResponse {
        let res = service.create(&greenhouse).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| HttpResponseBuilder::new(StatusCode::CREATED).json(GreenhouseDTO::from(greenhouse))
                )
}

#[utoipa::path(tag = GREENHOUSE_TAG)]
#[get("")]
pub async fn list_greenhouse(service: Data<dyn GreenhouseService>) -> HttpResponse {
        let res = service.list().await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouses| {
                                HttpResponseBuilder::new(StatusCode::OK)
                                        .json(greenhouses.into_iter().map(GreenhouseDTO::from).collect::<Vec<GreenhouseDTO>>())
                        }
                )
}

#[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO, IsSoftDTO))]
#[delete("/{id}")]
pub async fn delete_greenhouse(service: Data<dyn GreenhouseService>, id: Path<IdDTO>, is_soft: Query<IsSoftDTO>) -> HttpResponse {
        let res = service.delete(&id, **is_soft).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| greenhouse.map(GreenhouseDTO::from).no_content_or_not_found()
                )
}

// #[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO))]
// #[patch("/{id}")]
// pub async fn update_greenhouse_salary(service: Data<dyn GreenhouseService>, id: Path<IdDTO>, humidity: Json<SalaryDTO>) -> HttpResponse {
//         let res = service.update_humidity(&id, &humidity).await;
//
//         res.map_err(|err| err.to_string())
//                 .map_or_internal_server_error(
//                         |greenhouse| greenhouse.map(GreenhouseDTO::from).ok_or_not_found()
//                 )
// }

#[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO))]
#[post("/{id}/restore")]
pub async fn restore_greenhouse(service: Data<dyn GreenhouseService>, id: Path<IdDTO>) -> HttpResponse {
        let res = service.restore(&id).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| greenhouse.map(GreenhouseDTO::from).ok_or_not_found()
                )
}

#[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO))]
#[post("/{id}/conditions")]
pub async fn add_condition_to_greenhouse(service: Data<dyn GreenhouseService>, id: Path<IdDTO>, condition: Json<ConditionDTO>) -> HttpResponse {
        let res = service.add_condition(&id, &condition).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| {
                                match greenhouse {
                                        Some(greenhouse) => HttpResponseBuilder::new(StatusCode::CREATED).json(GreenhouseDTO::from(greenhouse)),
                                        None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
                                }
                        }
                )
}

#[utoipa::path(tag = GREENHOUSE_TAG, params(IdDTO))]
#[delete("/{id}/conditions")]
pub async fn remove_condition_from_greenhouse(service: Data<dyn GreenhouseService>, id: Path<IdDTO>, condition: Json<ConditionDTO>) -> HttpResponse {
        let res = service.remove_condition(&id, &condition).await;

        res.map_err(|err| err.to_string())
                .map_or_internal_server_error(
                        |greenhouse| greenhouse.map(GreenhouseDTO::from).no_content_or_not_found()
                )
}

