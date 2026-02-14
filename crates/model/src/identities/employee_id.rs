use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmployeeId(Uuid);

impl EmployeeId {
        pub fn new() -> Self {
                let id = Uuid::new_v7(Timestamp::now(NoContext));
                
                Self(id)
        }
}

impl From<Uuid> for EmployeeId {
        fn from(value: Uuid) -> Self {
                Self(value)
        }
}

impl Display for EmployeeId {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0.to_string())
        }
}