// super
use super::collect_items;

// Crate
use crate::objects::{kingdom::Kingdom, towns::Town};

use std::collections::HashSet;
// std
use std::io::{Error, ErrorKind, Result};
use std::{fs::OpenOptions, path::PathBuf};

pub(super) fn add_town_to_kingdom_i(
    town_index: usize,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    let kingdoms_path = PathBuf::new().join(&directory).join("kingdoms.json");
    let towns_path = PathBuf::new().join(directory).join("towns.json");

    let kingdoms_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(kingdoms_path)?;
    let towns_file = OpenOptions::new().read(true).write(true).open(towns_path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&kingdoms_file)?;
    let mut towns: Vec<Town> = collect_items(&towns_file)?;

    if kingdom_index == 0 || kingdom_index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid kingdom index"));
    }

    if town_index == 0 || town_index > towns.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid town index"));
    }

    let kingdom = kingdoms.get_mut(kingdom_index - 1).unwrap();
    let town = towns.get_mut(town_index - 1).unwrap();

    kingdom.towns.push(town.id.clone());
    town.has_kingdom = true;

    kingdoms_file.set_len(0)?;
    towns_file.set_len(0)?;

    serde_json::to_writer_pretty(kingdoms_file, &kingdoms)?;
    serde_json::to_writer_pretty(towns_file, &towns)?;

    Ok(())
}

pub(super) fn add_towns_to_kingdom_i(
    towns_index: Vec<usize>,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    let kingdoms_path = PathBuf::new().join(&directory).join("kingdoms.json");
    let towns_path = PathBuf::new().join(directory).join("towns.json");

    let kingdoms_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(kingdoms_path)?;
    let towns_file = OpenOptions::new().read(true).write(true).open(towns_path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&kingdoms_file)?;
    let mut towns: Vec<Town> = collect_items(&towns_file)?;

    let unique_town_ids: Vec<usize> = HashSet::<usize>::from_iter(towns_index)
        .into_iter()
        .collect();

    if kingdom_index == 0 || kingdom_index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid kingdom index"));
    }

    if unique_town_ids.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Towns index is empty"));
    }

    for &town_index in unique_town_ids.iter() {
        if town_index == 0 || town_index > towns.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid town index"));
        }
    }

    let kingdom = kingdoms.get_mut(kingdom_index - 1).unwrap();

    for i in unique_town_ids {
        let town = towns.get_mut(i - 1).unwrap();

        kingdom.towns.push(town.id.clone());
        town.has_kingdom = true;
    }

    kingdoms_file.set_len(0)?;
    towns_file.set_len(0)?;

    serde_json::to_writer_pretty(kingdoms_file, &kingdoms)?;
    serde_json::to_writer_pretty(towns_file, &towns)?;

    Ok(())
}

pub(super) fn delete_town_from_kingdom_i(
    town_index: usize,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    let kingdoms_path = PathBuf::new().join(&directory).join("kingdoms.json");
    let towns_path = PathBuf::new().join(directory).join("towns.json");

    let kingdoms_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(kingdoms_path)?;
    let towns_file = OpenOptions::new().read(true).write(true).open(towns_path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&kingdoms_file)?;
    let mut towns: Vec<Town> = collect_items(&towns_file)?;

    if kingdom_index == 0 || kingdom_index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid kingdom index"));
    }

    if town_index == 0 || town_index > towns.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid town index"));
    }

    let kingdom = kingdoms.get_mut(kingdom_index - 1).unwrap();
    let town = towns.get_mut(town_index - 1).unwrap();

    let id_index = kingdom.towns.iter().position(|id| id == &town.id);

    kingdom.towns.remove(id_index.unwrap());
    town.has_kingdom = false;

    kingdoms_file.set_len(0)?;
    towns_file.set_len(0)?;

    serde_json::to_writer_pretty(kingdoms_file, &kingdoms)?;
    serde_json::to_writer_pretty(towns_file, &towns)?;

    Ok(())
}
