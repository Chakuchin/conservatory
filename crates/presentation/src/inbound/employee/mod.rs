use utoipa_actix_web::{scope, service_config::ServiceConfig};
use crate::inbound::employee::handlers::{create_employee, delete_employee, get_employee, list_employees, restore_employee, update_employee_salary};

pub mod dto;
pub mod handlers;

pub fn configure(cfg: &mut ServiceConfig) {
        cfg.service(
                scope::scope("/employee")
                        .service(get_employee)
                        .service(create_employee)
                        .service(list_employees)
                        .service(delete_employee)
                        .service(update_employee_salary)
                        .service(restore_employee)
        );
}