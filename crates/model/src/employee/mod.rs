pub mod salary;

use salary::Salary;
use conservatory_core::id::Id;
use time::Date;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EmployeeModel {
        pub id: Id,
        pub name: String,
        pub surname: String,
        pub patronymic: Option<String>,
        pub salary: Salary,
        pub works_since: Date
}

impl EmployeeModel {
        pub fn new(
                name: String, surname: String, patronymic: Option<String>,
                salary: Salary, works_since: Date
        ) -> Self {
                Self {
                        id: Id::new(),
                        name,
                        surname,
                        patronymic,
                        salary,
                        works_since
                }
        }
}