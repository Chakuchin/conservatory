mod outbound;
mod open_api;
mod inbound;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use conservatory_application::services::employee::BaseEmployeeService;
use conservatory_model::services::employee::EmployeeService;
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use conservatory_application::services::greenhouse::BaseGreenhouseService;
use conservatory_application::services::plant::BasePlantService;
use conservatory_model::services::greenhouse::GreenhouseService;
use conservatory_model::services::plant::PlantService;
use crate::inbound::{employee, greenhouse, plant};
use crate::open_api::ApiDoc;
use crate::outbound::db;

static PORT: LazyLock<u16> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "PORT",
                8080
        )
);

static IP: LazyLock<Ipv4Addr> = LazyLock::new(||
        conservatory_core::env::resolve_or(
                "IP",
                Ipv4Addr::new(0, 0, 0, 0)
        )
);

fn main_configure(config: &mut ServiceConfig) {
        config.service(
                scope::scope("/api/v1")
                        .configure(employee::configure)
                        .configure(greenhouse::configure)
                        .configure(plant::configure)
        );
}

#[actix_web::main]
pub async fn main() -> Result<(), anyhow::Error> {
        dotenvy::dotenv().ok();

        simple_logger::init_with_level(log::Level::Info)?;

        actix_web::rt::time::sleep(Duration::from_secs(2)).await;

        let addr = SocketAddrV4::new(*IP, *PORT);

        let db = db::init().await;
        let employee_service: Arc<dyn EmployeeService> = Arc::new(BaseEmployeeService::new(db.clone()));
        let greenhouse_service: Arc<dyn GreenhouseService> = Arc::new(BaseGreenhouseService::new(db.clone()));
        let plant_service: Arc<dyn PlantService> = Arc::new(BasePlantService::new(db.clone()));
        let employee_data: Data<dyn EmployeeService> = Data::from(employee_service);
        let greenhouse_data: Data<dyn GreenhouseService> = Data::from(greenhouse_service);
        let plant_data: Data<dyn PlantService> = Data::from(plant_service);

        let server = HttpServer::new(
                        move || {
                                App::new()
                                        .into_utoipa_app()
                                        .app_data(employee_data.clone())
                                        .app_data(greenhouse_data.clone())
                                        .app_data(plant_data.clone())
                                        .openapi(ApiDoc::openapi())
                                        .map(|app| app.wrap(Logger::default()))
                                        .configure(main_configure)
                                        .openapi_service(open_api::swagger::init)
                                        .into_app()
                        }
                )
                .workers(2)
                .bind(addr)?;

        server.run().await?;

        Ok(())
}
