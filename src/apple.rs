use crate::frame::{Drawable, Frame, Pixel, X_NUM_COLS, Y_NUM_ROWS};
use fastrand;

pub struct Apple {
    x: u16,
    y: u16,
}

impl Apple {
    pub fn new() -> Self {
        let mut apple = Apple { x: 0, y: 0 };
        apple.relocate();
        apple
    }
    pub fn relocate(&mut self) {
        self.x = fastrand::u16(0..X_NUM_COLS);
        self.y = fastrand::u16(0..Y_NUM_ROWS);
    }
    pub fn is_at(&mut self, x: u16, y: u16) -> bool {
        self.x == x && self.y == y
    }
}

impl Drawable for Apple {
    fn draw(&self, frame: &mut Frame) {
        frame.put(self.x, self.y, Pixel::Apple);
    }
}
