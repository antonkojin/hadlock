use crate::xlibwrapper::util::*;
use crate::xlibwrapper::xlibmodels::Geometry;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    position: Position,
    size: Size,
}

impl Rect {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }
}

impl From<Geometry> for Rect {
    fn from(item: Geometry) -> Self {
        Self {
            position: Position {
                x: item.x,
                y: item.y,
            },
            size: Size {
                width: item.width as i32,
                height: item.height as i32,
            },
        }
    }
}
