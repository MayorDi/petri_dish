pub mod direction;

use crate::world::WIDTH_WORLD;

pub use direction::Direction;

#[derive(derive_new::new, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub trait GetPosition: Sized {
    fn get_position(&self) -> Position;
}

pub trait ArrayPosition: Sized {
    fn get_idx_pos(&self) -> usize where Self : GetPosition {
        let (x, y) = GetPosition::get_position(self).into();

        y * WIDTH_WORLD + x
    }

    fn get_pos_idx(&self) -> Position where Self : GetPosition {
        let index = self.get_idx_pos();

        Position::from((index % WIDTH_WORLD, index / WIDTH_WORLD))
    }
}
