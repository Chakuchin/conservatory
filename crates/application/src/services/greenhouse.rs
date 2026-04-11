use anyhow::Error;
use async_trait::async_trait;
use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use conservatory_model::di::unit_of_work::UnitOfWork;
use conservatory_model::enums::Condition;
use conservatory_model::greenhouse::GreenhouseModel;
use conservatory_model::providers::sql::SQLProvider;
use conservatory_model::repositories::sql::greenhouse::GreenhouseRepository;
use conservatory_model::services::greenhouse::GreenhouseService;

#[derive(Debug)]
pub struct BaseGreenhouseService<DB: SQLProvider> {
        db_provider: DB
}

impl<DB: SQLProvider> BaseGreenhouseService<DB> {
        pub fn new(db_provider: DB) -> Self {
                Self { db_provider }
        }
}

#[async_trait]
impl<DB: SQLProvider> GreenhouseService for BaseGreenhouseService<DB> {
        async fn create(&self, greenhouse: &GreenhouseModel) -> Result<GreenhouseModel, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let new_greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.create(greenhouse).await?
                };

                unit_of_work.commit().await?;

                Ok(new_greenhouse)
        }

        async fn get(&self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.get(id).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn list(&self) -> Result<Vec<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouses = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.list().await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouses)
        }

        async fn update_humidity(&self, id: &Id, humidity: RelativeHumidity) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.update_humidity(id, humidity).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn update_target_temperature(&self, id: &Id, temperature: Temperature<Celsius>) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.update_target_temperature(id, temperature).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn delete(&self, id: &Id, is_soft: bool) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        if is_soft {
                                repo.soft_delete(id).await?
                        } else {
                                repo.delete(id).await?
                        }
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn restore(&self, id: &Id) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.restore(id).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn add_condition(&self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.add_condition(id, condition).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }

        async fn remove_condition(&self, id: &Id, condition: &Condition) -> Result<Option<GreenhouseModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let greenhouse = {
                        let mut repo = unit_of_work.greenhouse_repo();
                        repo.remove_condition(id, condition).await?
                };

                unit_of_work.commit().await?;

                Ok(greenhouse)
        }
}