use crate::block::Block;
use ron::de::from_reader;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Error as IoError},
    path::Path,
};

pub fn map_from_file(file_path: impl AsRef<Path>) -> Result<HashMap<(i32, i32), Block>, IoError> {
    let f = File::open(&file_path)?;
    let map = from_reader(f).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to deserialize map: {}", err),
        )
    })?;
    Ok(map)
}