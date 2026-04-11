use weather_utils::{RelativeHumidity, Temperature};
use weather_utils::unit::Celsius;
use conservatory_core::id::Id;
use fixed::FixedU32;
use fixed::types::extra::U2;
use crate::enums::Condition;

pub type SquareMeters = FixedU32<U2>;

#[derive(Debug, Clone, PartialEq)]
pub struct GreenhouseModel {
        pub id: Id,
        pub name: String,
        pub humidity: RelativeHumidity,
        pub target_temperature: Temperature<Celsius>,
        pub conditions: Vec<Condition>,
        pub area: SquareMeters
}

impl GreenhouseModel {
        pub fn new(id: Id, name: &str, humidity: RelativeHumidity, temperature: Temperature<Celsius>, conditions: &[Condition], area: SquareMeters) -> Self {
                Self {
                        id,
                        name: name.to_string(),
                        humidity,
                        target_temperature: temperature,
                        conditions: conditions.to_vec(),
                        area
                }
        }
}