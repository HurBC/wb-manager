use crate::utils;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Leader {
    pub id: String,
    pub name: String,
    pub personality: String,
    pub has_kingdom: bool,
}

impl Leader {
    pub fn new(name: String, personality: String) -> Leader {
        Leader {
            id: utils::generate_random_string().unwrap(),
            name,
            personality,
            has_kingdom: false,
        }
    }
}

impl fmt::Debug for Leader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Name: {}, Personality: {}]",
            self.name, self.personality
        )
    }
}
