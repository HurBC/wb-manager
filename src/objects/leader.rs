use core::fmt;

use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Leader {
    pub id: String,
    pub name: String,
    pub personality: String,
    pub has_kingdom: bool,
    pub is_alive: bool,
}

impl Leader {
    pub fn new(name: String, personality: String) -> Leader {
        Leader {
            id: utils::generate_random_string().unwrap(),
            name,
            personality,
            has_kingdom: false,
            is_alive: true,
        }
    }
}

impl fmt::Display for Leader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[name: {}, personality: {}]",
            self.name, self.personality
        )
    }
}
