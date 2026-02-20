use std::str::FromStr;
use time::{Date, Month};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use conservatory_application::services::employee::BaseEmployeeService;
use conservatory_infrastructure::sql::providers::postgres::PostgresqlProvider;
use conservatory_model::employee::EmployeeModel;
use conservatory_model::employee::salary::Salary;
use conservatory_model::enums::Currency;
use conservatory_model::services::employee::EmployeeService;
use opt::RuntimeCommand;
use uuid::Uuid;
use cruet::to_pascal_case;
use crate::db::postgres::run_database_command;
use crate::opt::{Commands, ConservatoryCli};

pub mod opt;
mod db;

pub async fn init(opt: ConservatoryCli) -> Result<PostgresqlProvider, anyhow::Error> {
        match opt.command {
                Commands::Database(database_opt) => Ok(run_database_command(database_opt).await?)
        }
}

pub async fn run(db: PostgresqlProvider) -> Result<(), anyhow::Error> {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        let mut stdout = tokio::io::stdout();

        loop {
                stdout.write_all(b"CLI conservatory/employee> ").await?;
                stdout.flush().await?;

                if let Some(line) = stdin.next_line().await? {
                        let command = RuntimeCommand::from_str(&line)?;
                        match command {
                                RuntimeCommand::Create => {
                                        stdout.write_all(b"POST conservatory/employee PAYLOAD name> ").await?;
                                        stdout.flush().await?;

                                        let name = stdin.next_line().await?.unwrap();

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD surname> ").await?;
                                        stdout.flush().await?;

                                        let surname = stdin.next_line().await?.unwrap();

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD patronymic> ").await?;
                                        stdout.flush().await?;

                                        let patronymic = stdin.next_line().await?.unwrap();

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD currency> ").await?;
                                        stdout.flush().await?;

                                        let currency = match Currency::from_str(&stdin.next_line().await?.unwrap()) {
                                                Ok(currency) => currency,
                                                Err(_) => {
                                                        log::warn!("Please use supported currency like RUB");

                                                        continue
                                                }
                                        };

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD amount> ").await?;
                                        stdout.flush().await?;

                                        let amount = match stdin.next_line().await?.unwrap().parse() {
                                                Ok(currency) => currency,
                                                Err(_) => {
                                                        log::warn!("Please enter real amount like 100000");

                                                        continue
                                                }
                                        };

                                        let salary = Salary::new(amount, currency);

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD works_since/year> ").await?;
                                        stdout.flush().await?;

                                        let year = match stdin.next_line().await?.unwrap().parse() {
                                                Ok(year) => year,
                                                Err(_) => {
                                                        log::warn!("Please enter real year like 2020");

                                                        continue
                                                }
                                        };

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD works_since/month> ").await?;
                                        stdout.flush().await?;

                                        let month = match Month::from_str(&to_pascal_case(&stdin.next_line().await?.unwrap())) {
                                                Ok(month) => month,
                                                Err(_) => {
                                                        log::warn!("Please enter real moth like MAY");

                                                        continue
                                                }
                                        };

                                        stdout.write_all(b"POST conservatory/employee PAYLOAD works_since/day> ").await?;
                                        stdout.flush().await?;

                                        let day = match stdin.next_line().await?.unwrap().parse() {
                                                Ok(day) => day,
                                                Err(_) => {
                                                        log::warn!("Please enter real month like MAY");

                                                        continue
                                                }
                                        };

                                        let works_since = match Date::from_calendar_date(year, month, day) {
                                                Ok(date) => date,
                                                Err(_) => {
                                                        log::warn!("Please enter real date like 2020 MAY 10");

                                                        continue
                                                }
                                        };

                                        let new_employee = EmployeeModel::new(
                                                name, surname, Some(patronymic), salary, works_since
                                        );

                                        let service = BaseEmployeeService::new(db.begin().await?);

                                        let employee = service.create(new_employee).await?;

                                        log::info!("CREATED {employee:?}")
                                },
                                RuntimeCommand::Get => {
                                        stdout.write_all(b"GET conservatory/employee/{id}> ").await?;
                                        stdout.flush().await?;

                                        let id = match Uuid::from_str(&stdin.next_line().await?.unwrap()) {
                                                Ok(id) => id,
                                                Err(_) => {
                                                        log::warn!("Please enter real id like 019c7c20-00b9-789b-a426-0a0de8beaa5c");

                                                        continue
                                                }
                                        };

                                        let service = BaseEmployeeService::new(db.begin().await?);

                                        let employee = service.get(id.into()).await?;

                                        match employee {
                                                Some(employee) => log::info!("{employee:?}"),
                                                None => log::warn!("Employee {id} does not exist")
                                        }
                                }
                                RuntimeCommand::List => {
                                        let service = BaseEmployeeService::new(db.begin().await?);

                                        let employees = service.list().await?;

                                        for employee in employees {
                                                log::info!("{employee:?}")
                                        }
                                }
                                RuntimeCommand::Update => {
                                        stdout.write_all(b"PATCH conservatory/employee/{id}> ").await?;
                                        stdout.flush().await?;

                                        let id = match Uuid::from_str(&stdin.next_line().await?.unwrap()) {
                                                Ok(id) => id,
                                                Err(_) => {
                                                        log::warn!("Please enter real id like 019c7c20-00b9-789b-a426-0a0de8beaa5c");

                                                        continue
                                                }
                                        };

                                        stdout.write_all(b"PATCH conservatory/employee/{id} PAYLOAD currency> ").await?;
                                        stdout.flush().await?;

                                        let currency = match Currency::from_str(&stdin.next_line().await?.unwrap()) {
                                                Ok(currency) => currency,
                                                Err(_) => {
                                                        log::warn!("Please use supported currency like RUB");

                                                        continue
                                                }
                                        };

                                        stdout.write_all(b"PATCH conservatory/employee/{id} PAYLOAD amount> ").await?;
                                        stdout.flush().await?;

                                        let amount = match stdin.next_line().await?.unwrap().parse() {
                                                Ok(currency) => currency,
                                                Err(_) => {
                                                        log::warn!("Please enter real amount like 100000");

                                                        continue
                                                }
                                        };

                                        let salary = Salary::new(amount, currency);

                                        let service = BaseEmployeeService::new(db.begin().await?);

                                        let employee = service.update_salary(id.into(), salary).await?;

                                        match employee {
                                                Some(employee) => log::info!("{employee:?}"),
                                                None => log::warn!("Employee {id} does not exist")
                                        }
                                }
                                RuntimeCommand::Delete => {
                                        stdout.write_all(b"DELETE conservatory/employee/{id}> ").await?;
                                        stdout.flush().await?;

                                        let id = match Uuid::from_str(&stdin.next_line().await?.unwrap()) {
                                                Ok(id) => id,
                                                Err(_) => {
                                                        log::warn!("Please enter real id like 019c7c20-00b9-789b-a426-0a0de8beaa5c");

                                                        continue
                                                }
                                        };

                                        let service = BaseEmployeeService::new(db.begin().await?);

                                        let employee = service.delete(id.into(), true).await?;

                                        match employee {
                                                Some(employee) => log::info!("{employee:?}"),
                                                None => log::warn!("Employee {id} does not exist")
                                        }
                                }
                                RuntimeCommand::Help => {
                                        log::info!("\
                                        help - print this massage \n\
                                        create - create new employee \n\
                                        get - get employee \n\
                                        list - list all employees \n\
                                        update - update employee's salary \n\
                                        delete - delete employee \n\
                                        exit - exit cli \n")
                                }
                                RuntimeCommand::Exit => break,
                                RuntimeCommand::Unknown(s) => log::info!("Unknown command: {s}"),
                                _ => todo!()
                        }
                }
        }

        Ok(())
}