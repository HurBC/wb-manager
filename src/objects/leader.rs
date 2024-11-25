use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Leader {
    name: String,
    personality: String,
}

impl Leader {
    pub fn new(name: String, personality: String) -> Leader {
        Leader { name, personality }
    }
}

impl fmt::Display for Leader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Name: {}, Personality: {}]",
            self.name, self.personality
        )
    }
}
