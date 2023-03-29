use crate::coordinate::{Coordinate, CoordinateController};
use termgame::{Game, Message};

/// The player struct
pub struct Player {
    pub icon: char,
    position: Coordinate,
    is_in_water: bool,
    blocks_in_water: i32,
    is_drowned: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            icon: '\u{2657}',
            position: Coordinate { x: 3, y: 3 },
            is_in_water: false,
            blocks_in_water: 0,
            is_drowned: false,
        }
    }

    pub fn move_in_water(&mut self, game: &mut Game) {
        if !self.is_in_water {
            self.is_in_water = true;
            self.blocks_in_water = 0;
        }

        self.blocks_in_water += 1;

        if self.blocks_in_water >= 10 {
            self.is_drowned = true;

            game.set_message(Some(Message {
                title: Some(String::from("Message")),
                text: (String::from("You are drowned.")),
            }));
        }
    }

    pub fn move_outside_water(&mut self) {
        self.is_in_water = false;
        self.blocks_in_water = 0;
    }

    pub fn is_drowned(&mut self) -> bool {
        self.is_drowned
    }
}

// The player should be able to move as a coordinate controller
impl CoordinateController for Player {
    fn move_by(&mut self, coordinate: &Coordinate) {
        self.position.x += coordinate.x;
        self.position.y += coordinate.y;
    }

    fn get_position(&self) -> Coordinate {
        Coordinate {
            x: self.position.x,
            y: self.position.y,
        }
    }

    fn get_x(&self) -> i32 {
        self.position.x
    }

    fn get_y(&self) -> i32 {
        self.position.y
    }
}
