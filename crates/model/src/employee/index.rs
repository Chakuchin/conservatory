use crate::employee::id::EmployeeId;

#[derive(Debug)]
pub enum EmployeeIndex {
        Id(EmployeeId)
}

impl From<EmployeeId> for EmployeeIndex {
        fn from(value: EmployeeId) -> Self {
                Self::Id(value)
        }
}