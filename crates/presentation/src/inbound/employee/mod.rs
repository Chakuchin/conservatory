use utoipa_actix_web::{scope, service_config::ServiceConfig};
use crate::inbound::employee::handlers::{create_employee, delete_employee, get_employee, list_employees};

pub mod dto;
pub mod handlers;

pub fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
                scope::scope("/employee")
                        .service(get_employee)
                        .service(create_employee)
                        .service(list_employees)
                        .service(delete_employee)
        );
}