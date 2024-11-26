// crate
use crate::objects::kingdom::Kingdom;

// super
use super::collect_items;
use super::k_l_services::{add_leader_to_kingdom_i};
use super::k_t_services::{
    add_town_to_kingdom_i, add_towns_to_kingdom_i, delete_town_from_kingdom_i,
};

// std
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::{fs::OpenOptions, path::PathBuf};

pub fn add_kingdom(new_kingdom: Kingdom, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("kingdoms.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&file)?;

    kingdoms.push(new_kingdom);

    serde_json::to_writer_pretty(file, &kingdoms)?;

    Ok(())
}

pub fn list_kingdoms(directory: String) -> Result<()> {
    let path = PathBuf::new().join(&directory).join("kingdoms.json");

    if let Some(parent) = path.parent() {
        if !fs::exists(parent)? {
            println!("Parent directory does not exist");

            return Err(Error::new(
                ErrorKind::NotFound,
                "Parent directory does not exist",
            ));
        }
    }

    let file = OpenOptions::new().read(true).open(path)?;

    let kingdoms: Vec<Kingdom> = collect_items(&file)?;

    if kingdoms.is_empty() {
        println!("No kingdoms found");
    } else {
        let mut index = 1;

        for kingdom in kingdoms {
            print!("{}: {}", index, kingdom.to_string(&directory));
            index += 1;
        }
    }

    Ok(())
}

pub fn delete_kingdom(index: usize, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("kingdoms.json");

    let file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&file)?;

    if index == 0 || index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
    }

    kingdoms.remove(index - 1);

    file.set_len(0)?;

    serde_json::to_writer_pretty(file, &kingdoms)?;

    Ok(())
}

pub fn get_kingdom(index: usize, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("kingdoms.json");

    let file = OpenOptions::new().read(true).open(path)?;

    let kingdoms: Vec<Kingdom> = collect_items(&file)?;

    if index == 0 || index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
    }

    let kingdom = kingdoms.get(index - 1).unwrap();

    println!("{}", kingdom);

    Ok(())
}

pub fn add_town_to_kingdom(
    town_index: usize,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    add_town_to_kingdom_i(town_index, kingdom_index, directory)
}

pub fn add_towns_to_kingdom(
    towns_index: Vec<usize>,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    add_towns_to_kingdom_i(towns_index, kingdom_index, directory)
}

pub fn delete_town_from_kingdom(
    town_index: usize,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    delete_town_from_kingdom_i(town_index, kingdom_index, directory)
}

pub fn add_leader_to_kingdom(
    leader_index: usize,
    kingdom_index: usize,
    directory: String,
) -> Result<()> {
    add_leader_to_kingdom_i(kingdom_index, leader_index, directory)
}
