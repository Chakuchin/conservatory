use std::collections::HashMap;
use std::time::Duration;
use bollard::config::{ContainerCreateBody, ContainerStateStatusEnum, HostConfig, PortBinding};
use bollard::Docker;
use bollard::query_parameters::{CreateContainerOptions, CreateImageOptions, RestartContainerOptions};
use futures_util::TryStreamExt;
use time::{Date, Month};
use tokio::time::sleep;
use conservatory_application::services::employee::BaseEmployeeService;
use conservatory_infrastructure::sql::providers::postgres::PostgresqlProvider;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::Currency;
use conservatory_model::repositories::employee::EmployeeRepository;
use conservatory_model::services::employee::EmployeeService;
use crate::opt::{DatabaseCommand, DatabaseConfig, DatabaseOpt};

pub async fn run_database_command(opt: DatabaseOpt) -> anyhow::Result<()> {
        let command = opt.command.clone();

        match command {
                DatabaseCommand::Init(config) => {
                        start_postgres(&config).await?;
                        sleep(Duration::from_secs(10)).await; // TODO: Позже рализую нормальное ожидание
                        let pg = PostgresqlProvider::new(&config.host, config.port, &config.username, &config.database).await?;
                        pg.init().await?;

                        let service = BaseEmployeeService::new(pg.begin().await?);

                        let employee = service.create(EmployeeModel::new(
                                "test".to_string(),
                                "test2".to_string(),
                                Some("test3".to_string()),
                                Salary::new(100, Currency::RUB),
                                Date::from_calendar_date(2025, Month::April, 4)?,
                        )).await?;

                        // Сервис создаётся дважды, так как он предназдначен для использования хендлерами в будующем и является скорее use case
                        let service = BaseEmployeeService::new(pg.begin().await?);

                        log::info!("{:?}", service.get(employee.id).await?);
                }
        }
        Ok(())
}

async fn start_postgres(config: &DatabaseConfig) -> anyhow::Result<()> {
        let docker = Docker::connect_with_local_defaults()?;
        let container_name = "postgres-conservatory";
        let image_name = "postgres:latest";

        let container_exists = docker.inspect_container(container_name, None).await.is_ok();

        if container_exists {
                docker.restart_container(
                        container_name,
                        Some(RestartContainerOptions {
                                t: Some(10),
                                signal: None
                        })
                ).await?;

                wait_until_starts(docker, container_name, Duration::from_secs(15)).await;

                return Ok(())
        }

        docker.create_image(
                Some(CreateImageOptions {
                        from_image: Some(image_name.to_string()),
                        ..Default::default()
                }),
                None,
                None,
                )
                .try_collect::<Vec<_>>() // Дожидаемся завершения загрузки всех слоев
                .await?;

        let mut port_bindings = HashMap::new();
        port_bindings.insert(
                format!("{}/tcp", config.port),
                Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(config.port.to_string()),
                }]),
        );

        let host_config = HostConfig {
                port_bindings: Some(port_bindings),
                ..Default::default()
        };

        let container_config = ContainerCreateBody {
                image: Some(String::from("postgres")),
                env: Some(vec![
                        "POSTGRES_HOST_AUTH_METHOD=trust".to_string(),
                        format!("POSTGRES_DB={}", config.database),
                        format!("POSTGRES_USER={}", config.username)
                ]),
                host_config: Some(host_config),
                ..Default::default()
        };

        docker.create_container(
                Some(CreateContainerOptions {
                        name: Some(String::from("postgres-conservatory")),
                        ..Default::default()
                }),
                container_config,
        ).await?;

        docker.start_container(
                container_name,
                None
        ).await?;

        wait_until_starts(docker, container_name, Duration::from_secs(30)).await;

        Ok(())
}

async fn wait_until_starts(docker: Docker, container_name: &str, mut wait_for: Duration) {
        loop {
                let status = docker.inspect_container(container_name, None).await;

                if status.is_ok_and(|container|
                        container.state.is_some_and(|state|
                                state.status.is_some_and(|s|
                                        s == ContainerStateStatusEnum::RUNNING
                                )
                        )
                ) {
                        break
                }

                wait_for = wait_for.saturating_sub(Duration::from_secs(1));
                if wait_for.is_zero() {
                        break
                }

                sleep(Duration::from_secs(1)).await;
        }
}