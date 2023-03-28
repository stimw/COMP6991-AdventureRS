use serde::Deserialize;
use adventurers_quest::BlockTrait;
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Block {
    Grass,
    Sand,
    Rock,
    Cinderblock,
    Flowerbush,
    Barrier,
    Water,
    Sign(String),
    Object(char),
}

impl BlockTrait for Block {}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Grass => write!(f, "Grass"),
            Block::Sand => write!(f, "Sand"),
            Block::Rock => write!(f, "Rock"),
            Block::Cinderblock => write!(f, "Cinderblock"),
            Block::Flowerbush => write!(f, "Flowers"),
            Block::Water => write!(f, "Water"),
            Block::Barrier => write!(f, "Barrier"),
            Block::Sign(_) => write!(f, "Sign"),
            Block::Object(ch) => write!(f, "Object called '{}'", ch),
        }
    }
}
