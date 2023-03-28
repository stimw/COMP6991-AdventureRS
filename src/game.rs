use crate::block::Block;
use crate::coordinate::{Coordinate, CoordinateController};
use crate::game_quest::{GameQuest, GameQuestEvent};
use crate::map::map_from_file;
use crate::player::Player;
use adventurers_quest::{EventType, Quest};
use std::collections::HashMap;
use std::{io::Error as IoError, path::Path};
use termgame::{
    Controller, Game, GameColor, GameEvent, GameStyle, KeyCode, Message, SimpleEvent,
    StyledCharacter, ViewportLocation,
};

/// Get the quest by the user input string
fn get_quest_by_string(quest: &str) -> Box<dyn Quest<GameQuestEvent>> {
    match quest {
        "q1" => GameQuest::Q1.get_guest(),
        "q2" => GameQuest::Q2.get_guest(),
        _ => GameQuest::Q3.get_guest(),
    }
}

/// The main game struct
pub struct MyGame {
    player: Player,
    map: HashMap<(i32, i32), Block>,
    quest: Box<dyn Quest<GameQuestEvent>>,
}

impl MyGame {
    pub fn new(file_path: impl AsRef<Path>, quest_num: &str) -> Result<Self, IoError> {
        let map = map_from_file(file_path)?;

        Ok(Self {
            player: Player::default(),
            map,
            quest: get_quest_by_string(quest_num),
        })
    }

    /// Handle the game movement
    pub fn game_move(&mut self, game: &mut Game, keycode: KeyCode) {
        // 1. Get the coordinate movement
        let coordinate_movement = match keycode {
            KeyCode::Up => Coordinate::up_offset(),
            KeyCode::Down => Coordinate::down_offset(),
            KeyCode::Left => Coordinate::left_offset(),
            KeyCode::Right => Coordinate::right_offset(),
            _ => return,
        };

        // 2. If next block is a barrier, don't move and return
        if self.check_next_block_barrier(&coordinate_movement) {
            return;
        }

        // 3. Move the player
        self.move_player(game, &coordinate_movement);

        // 4. Check if the player is in the viewport
        if !self.check_if_in_viewport(game) {
            // If not, move the viewport
            let mut viewport_location = game.get_viewport();
            viewport_location.x += coordinate_movement.x;
            viewport_location.y += coordinate_movement.y;
            game.set_viewport(viewport_location);
        }

        // 5. Check if the player is in water or on a sign
        match self.get_current_block(&self.player.get_position()) {
            Some(Block::Water) => self.player.move_in_water(game),
            Some(Block::Sign(message)) => {
                game.set_message(Some(Message {
                    title: Some(String::from("Message")),
                    text: message.clone(),
                }));
                self.player.move_out_of_water()
            }
            _ => self.player.move_out_of_water(),
        };
    }
}

// Helpful methods
impl MyGame {
    /// Initialize the map
    fn init_map(&mut self, game: &mut Game) {
        for ((x, y), block) in &self.map {
            let styled_char = match block {
                Block::Grass => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Green))),
                Block::Sand => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
                Block::Rock => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Gray))),
                Block::Cinderblock => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Red))),
                Block::Flowerbush => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
                Block::Barrier => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::White))),
                Block::Water => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Blue))),
                Block::Sign(_) => StyledCharacter::new('💬'),
                Block::Object(character) => StyledCharacter::new(*character),
            };
            game.set_screen_char(*x, *y, Some(styled_char));
        }
    }

    /// Initialize the player
    fn init_player(&mut self, game: &mut Game) {
        self.add_player_to_screen(game);
    }

    /// Get the current block
    pub fn get_current_block(&self, coordinate: &Coordinate) -> Option<&Block> {
        self.map.get(&(coordinate.x, coordinate.y))
    }

    /// Move the player
    fn move_player(&mut self, game: &mut Game, coordinate_movement: &Coordinate) {
        self.remove_player_from_screen(game);
        self.player.move_by(coordinate_movement);
        self.add_player_to_screen(game);
    }

    fn remove_player_from_screen(&self, game: &mut Game) {
        let styled_char = game.get_screen_char(self.player.get_x(), self.player.get_y());
        if let Some(style) = styled_char {
            let character = match self.get_current_block(&self.player.get_position()) {
                Some(Block::Sign(_)) => '💬',
                _ => ' ',
            };
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(style.character(character)),
            );
        } else {
            game.set_screen_char(self.player.get_x(), self.player.get_y(), None);
        }
    }

    fn add_player_to_screen(&self, game: &mut Game) {
        let styled_char = game.get_screen_char(self.player.get_x(), self.player.get_y());
        if let Some(style) = styled_char {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(style.character(self.player.icon)),
            );
        } else {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(StyledCharacter::from(self.player.icon)),
            );
        }
    }

    /// Check if the player is in the viewport after moving
    fn check_if_in_viewport(&self, game: &Game) -> bool {
        let ViewportLocation { x: vp_x, y: vp_y } = game.get_viewport();
        let Coordinate {
            x: player_x,
            y: player_y,
        } = self.player.get_position();
        (vp_x..vp_x + 77).contains(&player_x) && (vp_y..vp_y + 21).contains(&player_y)
    }

    /// Check if the next block is a barrier
    fn check_next_block_barrier(&self, coordinate_movement: &Coordinate) -> bool {
        let current_block = self.player.get_position();
        matches!(
            self.map.get(&(
                current_block.x + coordinate_movement.x,
                current_block.y + coordinate_movement.y
            )),
            Some(Block::Barrier)
        )
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        // Initialize the map
        self.init_map(game);
        // Initialize the player
        self.init_player(game);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        // Check previous messages first
        // If the player is drowned or won the game, end the game
        if let Some(Message { text, .. }) = game.get_message() {
            if matches!(text.as_str(), "You are drowned." | "YOU WIN!") {
                game.end_game();
            } else {
                game.set_message(None);
            }
        }

        // Handle KeyCode Events
        if let SimpleEvent::Just(key_code) = event.into() {
            match key_code {
                // If the player presses 'q', show the quest
                KeyCode::Char('q') => {
                    game.set_message(Some(Message {
                        title: Some("Quest".to_string()),
                        text: format!("{}", self.quest),
                    }));
                }

                // If the player presses 'r', reset the quest
                KeyCode::Char('r') => {
                    self.quest.reset();
                }
                
                // Otherwise, move the player according to the key pressed
                _ => {
                    // handle movement
                    self.game_move(game, key_code);

                    // generate event
                    let current_block = self
                        .get_current_block(&self.player.get_position())
                        .unwrap_or(&Block::Barrier);
                    let event = match current_block {
                        Block::Object(_) => GameQuestEvent {
                            block: current_block.clone(),
                            event_type: EventType::Collect,
                        },
                        _ => GameQuestEvent {
                            block: current_block.clone(),
                            event_type: EventType::Walk,
                        },
                    };

                    // register event
                    if let adventurers_quest::QuestStatus::Complete = 
                        self.quest.register_event(&event)
                    {
                        game.set_message(Some(Message {
                            title: Some(String::from("Quest")),
                            text: String::from("YOU WIN!"),
                        }));
                    }
                }
            }
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}
