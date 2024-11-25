use serde::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Result, Seek, SeekFrom};

pub fn collect_items<TItems: for<'de> Deserialize<'de> + Debug>(
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
