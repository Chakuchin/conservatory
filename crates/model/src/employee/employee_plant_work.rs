use conservatory_core::id::Id;
use crate::enums::WorkType;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EmployeePlantWorkModel {
        pub id: Id,
        pub employee_id: Id,
        pub plant_id: Id,
        pub work_type: WorkType
}

impl EmployeePlantWorkModel {
        pub fn new(id: Id, employee_id: Id, plant_id: Id, work_type: WorkType) -> Self {
                Self {
                        id,
                        employee_id,
                        plant_id,
                        work_type
                }
        }
}