use uuid::fmt::Urn;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PlantTypeModel {
        pub urn: Urn,
        pub name: String,
        pub description: String
}

impl PlantTypeModel {
        pub fn new(urn: Urn, name: String, description: String) -> Self {
                Self {
                        urn,
                        name,
                        description
                }
        }
}