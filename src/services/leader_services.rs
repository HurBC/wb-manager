// Crate imports
use crate::objects::leader::Leader;

// Super imports
use super::collect_items;

// Std imports
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::{fs::OpenOptions, path::PathBuf};

pub fn add_leader(new_leader: Leader, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("leaders.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut leaders: Vec<Leader> = collect_items(&file)?;

    leaders.push(new_leader);

    serde_json::to_writer_pretty(&file, &leaders)?;

    Ok(())
}

pub fn get_leader_by_id(id: &String, directory: &String) -> Result<Leader> {
    let path = PathBuf::new().join(directory).join("leaders.json");

    let file = OpenOptions::new().read(true).open(path)?;

    let leaders: Vec<Leader> = collect_items(&file)?;

    let leader = leaders.into_iter().find(|l| &l.id == id);

    match leader {
        Some(leader) => Ok(leader),
        None => Err(Error::new(ErrorKind::NotFound, "Leader not found")),
    }
}

pub fn delete_leader(index: usize, directory: String) -> Result<()> {
    let path = PathBuf::new().join(directory).join("leaders.json");

    let file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut leaders: Vec<Leader> = collect_items(&file)?;

    if index == 0 || index > leaders.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
    }

    leaders.remove(index - 1);

    file.set_len(0)?;

    serde_json::to_writer_pretty(&file, &leaders)?;

    Ok(())
}
