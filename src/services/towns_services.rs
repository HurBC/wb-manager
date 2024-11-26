// Crate imports
use crate::objects::towns::Town;

// Super
use super::collect_items;

// std
use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub fn add_town(new_town: Town, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("towns.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut towns: Vec<Town> = collect_items(&file)?;

    towns.push(new_town);

    serde_json::to_writer_pretty(file, &towns)?;

    Ok(())
}

pub fn list_towns(directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("towns.json");

    let file = OpenOptions::new().read(true).open(path)?;

    let towns: Vec<Town> = collect_items(&file)?;

    if towns.is_empty() {
        println!("No towns found");
    } else {
        let mut index = 1;

        for town in towns {
            println!("{}: {}", index, town);

            index += 1;
        }
    }

    Ok(())
}

pub fn delete_town(index: usize, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("towns.json");

    let file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut towns: Vec<Town> = collect_items(&file)?;

    if index == 0 || index > towns.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
    }

    towns.remove(index - 1);

    file.set_len(0)?;

    serde_json::to_writer_pretty(file, &towns)?;

    Ok(())
}

pub fn get_town(index: usize, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("towns.json");

    let file = OpenOptions::new().read(true).open(path)?;

    let towns: Vec<Town> = collect_items(&file)?;

    if index == 0 || index > towns.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
    }

    let town = towns.get(index - 1).unwrap();

    println!("{:#?}", town);

    Ok(())
}

pub fn get_town_by_id(id: String, directory: &String) -> Result<Town> {
    let path = PathBuf::new().join(directory).join("towns.json");

    let file = OpenOptions::new().read(true).open(path)?;

    let towns: Vec<Town> = collect_items(&file)?;

    let town = towns
        .into_iter()
        .find(|town| town.id == id)
        .ok_or(Error::new(ErrorKind::NotFound, "Town not found"))?;

    Ok(town)
}
