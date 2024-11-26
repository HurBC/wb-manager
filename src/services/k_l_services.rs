// Crate Imports
use crate::objects::kingdom::Kingdom;
use crate::objects::leader::Leader;

// Super imports
use super::collect_items;

// Std imports
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub(super) fn add_leader_to_kingdom_i(
    kingdom_index: usize,
    leader_index: usize,
    directory: String,
) -> Result<()> {
    let kingdoms_path = PathBuf::new().join(&directory).join("kingdoms.json");
    let leaders_path = PathBuf::new().join(directory).join("leaders.json");

    let kingdoms_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(kingdoms_path)?;
    let leaders_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(leaders_path)?;

    let mut kingdoms: Vec<Kingdom> = collect_items(&kingdoms_file)?;
    let mut leaders: Vec<Leader> = collect_items(&leaders_file)?;

    if kingdom_index == 0 || kingdom_index > kingdoms.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid kingdom index"));
    }

    if leader_index == 0 || leader_index > leaders.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid town index"));
    }

    let kingdom = kingdoms.get_mut(kingdom_index - 1).unwrap();
    let leader = leaders.get_mut(leader_index - 1).unwrap();

    kingdom.leader = Some(leader.id.clone());
    leader.has_kingdom = true;

    kingdoms_file.set_len(0)?;
    leaders_file.set_len(0)?;

    serde_json::to_writer_pretty(kingdoms_file, &kingdoms)?;
    serde_json::to_writer_pretty(leaders_file, &leaders)?;

    Ok(())
}
