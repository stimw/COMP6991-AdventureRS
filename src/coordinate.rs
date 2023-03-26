#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn stay_still() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_up() -> Self {
        Self { x: 0, y: -1 }
    }

    pub fn move_down() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn move_left() -> Self {
        Self { x: -1, y: 0 }
    }

    pub fn move_right() -> Self {
        Self { x: 1, y: 0 }
    }
}

pub trait CoordinateController {
    fn move_by(&mut self, coordinate: &Coordinate);
    fn get_position(&self) -> Coordinate;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}
