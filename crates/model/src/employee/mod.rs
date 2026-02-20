pub mod salary;
pub mod id;
pub mod index;

use salary::Salary;
use id::EmployeeId;
use time::Date;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EmployeeModel {
        pub id: EmployeeId,
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
                        id: EmployeeId::new(),
                        name,
                        surname,
                        patronymic,
                        salary,
                        works_since
                }
        }
}