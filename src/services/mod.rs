use serde::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Result, Seek, SeekFrom};

// Self modules
pub(self) mod k_l_services;
pub(self) mod k_t_services;

// Public modules
pub mod kingdom_services;
pub mod leader_services;
pub mod towns_services;

pub(self) fn collect_items<TItems: for<'de> Deserialize<'de> + Debug>(
    mut file: &File,
) -> Result<Vec<TItems>> {
    file.seek(SeekFrom::Start(0))?;

    let items: Vec<TItems> = match serde_json::from_reader(file) {
        Ok(items) => items,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(items)
}
