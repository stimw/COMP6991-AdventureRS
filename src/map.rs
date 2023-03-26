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

#[test]
fn test_read_map() {
    let file_path = "test_map.ron";
    let map_data = r#"
        {
            (0, 0): Grass,
            (1, 0): Rocks,
            (0, 1): Sand,
            (1, 1): Flowerbush
        }
    "#;
    std::fs::write(file_path, map_data).unwrap();

    let expected_map: HashMap<(i32, i32), Block> = [
        ((0, 0), Block::Grass),
        ((1, 0), Block::Rock),
        ((0, 1), Block::Sand),
        ((1, 1), Block::Flowerbush),
    ]
    .iter()
    .cloned()
    .collect();

    let actual_map = map_from_file(file_path).unwrap();

    assert_eq!(actual_map, expected_map);

    // std::fs::remove_file(file_path).unwrap();
}
