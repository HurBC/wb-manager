// crate
use crate::services::towns_services::get_town_by_id;
use crate::utils;

// super
use super::leader::Leader;
use super::towns::Town;

// others
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kingdom {
    pub id: String,
    pub army: u32,
    pub name: String,
    pub leader: Option<Leader>,
    pub towns: Vec<String>,
    directory: Option<String>,
}

impl Kingdom {
    pub fn new(name: String) -> Kingdom {
        Kingdom {
            id: utils::generate_random_string().unwrap(),
            name,
            army: 0,
            leader: None,
            towns: Vec::new(),
            directory: None,
        }
    }

    pub fn to_string(&self, directory: &String) -> String {
        let mut message = String::new();

        let mut towns = self.get_towns(directory);

        towns.sort_by(|a, b| a.population.cmp(&b.population));

        let towns: Vec<&Town> = towns.iter().take(3).collect();

        message.push_str(
            format!(
                "{}: (army: {}, leader: {:?}, towns total: {}, top towns: {:?})\n",
                self.name,
                self.army,
                self.leader,
                self.towns.len(),
                towns
            )
            .as_str(),
        );

        message
    }

    pub(super) fn get_towns(&self, directory: &String) -> Vec<Town> {
        let mut towns: Vec<Town> = Vec::new();

        for id in self.towns.iter() {
            let town = get_town_by_id(id.clone(), &directory).unwrap();

            towns.push(town);
        }

        towns
    }
}

impl fmt::Display for Kingdom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: (army: {}, leader: {:?}, towns: {})",
            self.name,
            self.army,
            self.leader,
            self.towns.len()
        )
    }
}
