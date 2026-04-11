use anyhow::Error;
use async_trait::async_trait;
use conservatory_core::id::{Id, TypeId};
use conservatory_model::di::unit_of_work::UnitOfWork;
use conservatory_model::employee::employee_plant_work::EmployeePlantWorkModel;
use conservatory_model::enums::WorkType;
use conservatory_model::plant::{PlantModel, PlantTypeModel};
use conservatory_model::providers::sql::SQLProvider;
use conservatory_model::repositories::sql::greenhouse::GreenhouseRepository;
use conservatory_model::repositories::sql::plant::PlantRepository;
use conservatory_model::services::plant::PlantService;

#[derive(Debug)]
pub struct BasePlantService<DB: SQLProvider> {
        db_provider: DB
}

impl<DB: SQLProvider> BasePlantService<DB> {
        pub fn new(db_provider: DB) -> Self {
                Self { db_provider }
        }
}

#[async_trait]
impl<DB: SQLProvider> PlantService for BasePlantService<DB> {
        async fn register(&self, plant_type: &PlantTypeModel) -> Result<PlantTypeModel, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let new_plant_type = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.register(plant_type).await?
                };

                unit_of_work.commit().await?;

                Ok(new_plant_type)
        }

        async fn create(&self, id: &Id, type_urn: &TypeId) -> Result<PlantModel, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let new_plant = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.create(id, type_urn).await?
                };

                unit_of_work.commit().await?;

                Ok(new_plant)
        }

        async fn get_type(&self, type_urn: &TypeId) -> Result<Option<PlantTypeModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant_type = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.get_type(type_urn).await?
                };

                unit_of_work.commit().await?;

                Ok(plant_type)
        }

        async fn get(&self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.get(plant_id).await?
                };

                unit_of_work.commit().await?;

                Ok(plant)
        }

        async fn list_types(&self) -> Result<Vec<PlantTypeModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant_types = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.list_types().await?
                };

                unit_of_work.commit().await?;

                Ok(plant_types)
        }

        async fn list(&self) -> Result<Vec<PlantModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plants = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.list().await?
                };

                unit_of_work.commit().await?;

                Ok(plants)
        }

        async fn update_type_description(&self, type_urn: &TypeId, description: &str) -> Result<Option<PlantTypeModel>, Error> {
                todo!()
        }

        async fn delete_plant(&self, plant_id: &Id, is_soft: bool) -> Result<Option<()>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant_type = {
                        let mut repo = unit_of_work.plant_repo();
                        if is_soft {
                                repo.soft_delete_plant(plant_id).await?.map(|_| ())
                        } else {
                                repo.delete_plant(plant_id).await?
                        }
                };

                unit_of_work.commit().await?;

                Ok(plant_type)
        }

        async fn delete_type(&self, type_urn: &TypeId) -> Result<Option<()>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant_type = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.delete_type(type_urn).await?
                };

                unit_of_work.commit().await?;

                Ok(plant_type)
        }

        async fn restore_plant(&self, plant_id: &Id) -> Result<Option<PlantModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.restore_plant(plant_id).await?
                };

                unit_of_work.commit().await?;

                Ok(plant)
        }

        async fn work_with(&self, employee_id: &Id, plant_id: &Id, work_type: &WorkType) -> Result<Option<EmployeePlantWorkModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let work = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.work_with(plant_id, employee_id, work_type).await?
                };

                unit_of_work.commit().await?;

                Ok(work)
        }

        async fn plant_at(&self, plant_id: &Id, employee_id: &Id, greenhouse_id: &Id) -> Result<Option<PlantModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.plant_at(plant_id, greenhouse_id).await?
                };

                unit_of_work.commit().await?;

                Ok(plant)
        }

        async fn remove_from(&self, plant_id: &Id, employee_id: &Id) -> Result<Option<PlantModel>, Error> {
                let mut unit_of_work = self.db_provider.begin().await?;

                let plant = {
                        let mut repo = unit_of_work.plant_repo();
                        repo.remove_from(plant_id).await?
                };

                unit_of_work.commit().await?;

                Ok(plant)
        }
}