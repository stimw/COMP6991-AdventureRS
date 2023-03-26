#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

// These are the offsets for the movement
impl Coordinate {
    pub fn up_offset() -> Self {
        Self { x: 0, y: -1 }
    }

    pub fn down_offset() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn left_offset() -> Self {
        Self { x: -1, y: 0 }
    }

    pub fn right_offset() -> Self {
        Self { x: 1, y: 0 }
    }
}

pub trait CoordinateController {
    fn move_by(&mut self, coordinate: &Coordinate);
    fn get_position(&self) -> Coordinate;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}
