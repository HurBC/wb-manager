use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn to_string(&self) -> String {
        format!("[name: {}, personality: {}]", self.name, self.personality)
    }
}
