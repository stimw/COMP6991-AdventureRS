use crate::block::Block;
use adventurers_quest::{AtomicQuest, ChoiceQuest, EventType, InOrderQuest, Quest, QuestEvent};

/// The game quest event
/// - block: the block that the player is interacting with, e.g. sand, water, object
/// - event_type: the type of event, e.g. walk, collect
pub struct GameQuestEvent {
    pub block: Block,
    pub event_type: EventType,
}

// The game quest event should be able to be used as a quest event
impl QuestEvent for GameQuestEvent {
    type Block = Block;

    fn get_type(&self) -> &EventType {
        &self.event_type
    }

    fn get_block(&self) -> &Self::Block {
        &self.block
    }
}

/// The game quest
/// - Q1: walk on sand 5 times
/// - Q2: walk on sand 5 times and then collect 3 objects called 'y'
/// - Q3: choose 2 in 3: 
///       1. walk on sand 5 times and then collect 1 object called 'x'; 
///       2. collect 1 object called 'y' and then walk on grass 1 time; 
///       3. walk over 9 blocks of water, 3 times
pub enum GameQuest {
    Q1,
    Q2,
    Q3,
}

impl GameQuest {
    /// Get the guest according to the enum member
    pub fn get_guest(&self) -> Box<dyn Quest<GameQuestEvent>> {
        match self {
            GameQuest::Q1 => Box::from(AtomicQuest::new(Block::Sand, EventType::Walk, 5, 1)),

            GameQuest::Q2 => Box::from(InOrderQuest::new(vec![
                Box::from(AtomicQuest::new(Block::Sand, EventType::Walk, 5, 1)),
                Box::from(AtomicQuest::new(
                    Block::Object('y'),
                    EventType::Collect,
                    3,
                    1,
                )),
            ])),

            GameQuest::Q3 => Box::from(ChoiceQuest::new(
                vec![
                    Box::from(InOrderQuest::new(vec![
                        Box::from(AtomicQuest::new(Block::Sand, EventType::Walk, 5, 1)),
                        Box::from(AtomicQuest::new(
                            Block::Object('x'),
                            EventType::Collect,
                            1,
                            1,
                        )),
                    ])),
                    Box::from(InOrderQuest::new(vec![
                        Box::from(AtomicQuest::new(
                            Block::Object('y'),
                            EventType::Collect,
                            1,
                            1,
                        )),
                        Box::from(AtomicQuest::new(Block::Grass, EventType::Walk, 1, 1)),
                    ])),
                    Box::from(AtomicQuest::new(Block::Water, EventType::Walk, 9, 3)),
                ],
                2,
            )),
        }
    }
}
