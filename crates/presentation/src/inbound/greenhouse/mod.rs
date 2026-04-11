use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::inbound::greenhouse::handlers::{add_condition_to_greenhouse, create_greenhouse, delete_greenhouse, get_greenhouse, list_greenhouse, remove_condition_from_greenhouse, restore_greenhouse};

pub mod dto;
pub mod handlers;

pub fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
                scope::scope("/greenhouse")
                        .service(get_greenhouse)
                        .service(create_greenhouse)
                        .service(list_greenhouse)
                        .service(delete_greenhouse)
                        .service(restore_greenhouse)
                        .service(add_condition_to_greenhouse)
                        .service(remove_condition_from_greenhouse)
        );
}