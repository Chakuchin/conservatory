use crate::enums::Currency;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Salary {
        pub amount: u32,
        pub currency: Currency
}

impl Salary {
        pub fn new(amount: u32, currency: Currency) -> Self {
                Self {
                        amount,
                        currency
                }
        }
}