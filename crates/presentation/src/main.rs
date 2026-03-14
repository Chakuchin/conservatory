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
use crate::inbound::employee;
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
        );
}

#[actix_web::main]
pub async fn main() -> Result<(), anyhow::Error> {
        dotenvy::dotenv().ok();

        simple_logger::init_with_level(log::Level::Info)?;

        actix_web::rt::time::sleep(Duration::from_secs(2)).await;

        let addr = SocketAddrV4::new(*IP, *PORT);

        let db = db::init().await;
        let service: Arc<dyn EmployeeService> = Arc::new(BaseEmployeeService::new(db.clone()));
        let data: Data<dyn EmployeeService> = Data::from(service);

        let server = HttpServer::new(
                        move || {
                                App::new()
                                        .into_utoipa_app()
                                        .app_data(data.clone())
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
