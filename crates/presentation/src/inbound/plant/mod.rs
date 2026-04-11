use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::inbound::plant::handlers::{restore_plant, create_plant, delete_plant, delete_plant_type, get_plant, get_plant_type, list_plant_types, list_plants, plant_at, register_plant_type, remove_from};

pub mod handlers;
pub mod dto;

pub fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
                scope::scope("/plant")
                        .service(create_plant)
                        .service(get_plant)
                        .service(list_plants)
                        .service(delete_plant)
                        .service(plant_at)
                        .service(remove_from)
                        .service(restore_plant)
        ).service(
                scope::scope("/plant_type")
                        .service(register_plant_type)
                        .service(get_plant_type)
                        .service(list_plant_types)
                        .service(delete_plant_type)
        );
}