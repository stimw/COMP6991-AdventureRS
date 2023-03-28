use crate::block::Block;
use adventurers_quest::{AtomicQuest, ChoiceQuest, EventType, InOrderQuest, Quest, QuestEvent};

pub struct GameQuestEvent {
    pub block: Block,
    pub event_type: EventType,
}

impl QuestEvent for GameQuestEvent {
    type Block = Block;

    fn get_type(&self) -> &EventType {
        &self.event_type
    }

    fn get_block(&self) -> &Self::Block {
        &self.block
    }
}

pub enum GameQuest {
    Q1,
    Q2,
    Q3,
}

impl GameQuest {
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
