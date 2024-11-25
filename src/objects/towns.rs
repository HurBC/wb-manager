use crate::utils;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Town {
    pub id: String,
    pub name: String,
    pub population: u32,
    pub loyalty: i32,
    pub has_kingdom: bool,
}

impl Town {
    pub fn new(name: String, population: u32) -> Town {
        Town {
            id: utils::generate_random_string().unwrap(),
            name,
            population,
            loyalty: 0,
            has_kingdom: false,
        }
    }
}

impl fmt::Display for Town {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ({})", self.name, self.population)
    }
}

impl fmt::Debug for Town {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: (population: {}, loyalty: {})",
            self.name, self.population, self.loyalty
        )
    }
}
