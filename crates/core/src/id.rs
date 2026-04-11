use std::fmt::{Display, Formatter};
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use uuid::{NoContext, Timestamp, Uuid, fmt::Urn};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
        pub fn new() -> Self {
                let id = Uuid::new_v7(Timestamp::now(NoContext));

                Self(id)
        }
}

impl From<Uuid> for Id {
        fn from(value: Uuid) -> Self {
                Self(value)
        }
}

impl Deref for Id {
        type Target = Uuid;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl Display for Id {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0.to_string())
        }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeId(Urn);

impl TypeId {
        pub fn new(urn: Urn) -> Self {
                Self(urn)
        }
}

impl From<Id> for TypeId {
        fn from(value: Id) -> Self {
                Self(value.urn())
        }
}

impl From<Urn> for TypeId {
        fn from(value: Urn) -> Self { Self(value) }
}

impl Deref for TypeId {
        type Target = Urn;

        fn deref(&self) -> &Self::Target {
                &self.0
        }
}

impl Display for TypeId {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0.to_string())
        }
}