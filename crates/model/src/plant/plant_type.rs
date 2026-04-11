use uuid::fmt::Urn;
use conservatory_core::id::TypeId;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PlantTypeModel {
        pub urn: TypeId,
        pub name: String,
        pub description: String
}

impl PlantTypeModel {
        pub fn new(urn: Urn, name: String, description: String) -> Self {
                Self {
                        urn: urn.into(),
                        name,
                        description
                }
        }
}