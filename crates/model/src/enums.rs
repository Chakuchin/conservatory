use strum::{Display, EnumString, AsRefStr};
use serde::{Serialize, Deserialize};

#[derive(
        Debug,
        Display,
        EnumString,
        AsRefStr,
        Clone,
        Hash,
        Eq,
        PartialEq,
        Serialize,
        Deserialize
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
        RUB,
        USD
}

#[derive(
        Debug,
        Display,
        EnumString,
        AsRefStr,
        Clone,
        Hash,
        Eq,
        PartialEq,
        Serialize,
        Deserialize
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Condition {
        RICH,
        ACIDIC,
        HIGH
}

#[derive(
        Debug,
        Display,
        EnumString,
        AsRefStr,
        Clone,
        Hash,
        Eq,
        PartialEq,
        Serialize,
        Deserialize
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkType {
        PLANT,
        WATER,
        FERTILIZE,
        CULTIVATE,
        UPROOT
}