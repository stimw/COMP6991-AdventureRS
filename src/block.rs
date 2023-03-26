use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Block {
    Grass,
    Sand,
    Rocks,
    Cinderblock,
    Flowers,
    Barrier,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Grass => write!(f, "Grass"),
            Block::Sand => write!(f, "Sand"),
            Block::Rocks => write!(f, "Rock"),
            Block::Cinderblock => write!(f, "Cinderblock"),
            Block::Flowers => write!(f, "Flowers"),
            _ => write!(f, "Other block"),
        }
    }
}
