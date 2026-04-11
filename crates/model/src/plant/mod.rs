use conservatory_core::id::Id;
pub use crate::plant::plant_type::PlantTypeModel;

pub mod plant_type;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PlantModel {
        pub id: Id,
        pub plant_type: PlantTypeModel,
        pub greenhouse_id: Option<Id>
}

impl PlantModel {
        pub fn new(id: Id, plant_type: PlantTypeModel, greenhouse_id: Option<Id>) -> Self {
                Self {
                        id,
                        plant_type,
                        greenhouse_id
                }
        }

        pub fn plant_at_greenhouse(self, greenhouse_id: Id) -> Self {
                Self {
                        id: self.id,
                        plant_type: self.plant_type,
                        greenhouse_id: Some(greenhouse_id)
                }
        }

        pub fn remove_from_greenhouse(self) -> Self {
                Self {
                        id: self.id,
                        plant_type: self.plant_type,
                        greenhouse_id: None
                }
        }
}