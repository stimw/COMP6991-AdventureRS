use crate::coordinate::{Coordinate, CoordinateController};

pub struct Player {
    pub icon: char,
    position: Coordinate,
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
        }
    }
}

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
