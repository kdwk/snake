use crate::{frame::{Drawable, Frame}, NUM_APPLES, NUM_ROWS, NUM_COLS};
use fastrand as random;

#[derive(Debug, Clone, Copy)]
pub struct Apple {
    pub x: usize,
    pub y: usize
}

impl Apple {
    fn new(x: usize, y: usize) -> Self {
        Self {x: x, y: y}
    }
}

// pub type Apples = Vec<Apple>;

#[derive(Debug, Clone, PartialEq)]
pub struct Apples {
    list: Vec<Apple>,
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !(self.x == other.x && self.y == other.y)
    }
}

impl Apples {
    fn init(&mut self) {
        for _ in 0..NUM_APPLES {
            let x = random::usize(0..NUM_COLS);
            let y = random::usize(0..NUM_ROWS);
            self.list.push(Apple::new(x, y));
        }
    }
    fn eat(&mut self, x: usize, y: usize) {
        self.list.retain(|&apple| apple == Apple::new(x, y));
    }
}

impl Drawable for Apples {
    fn draw(self, frame: &mut Frame) {
        for apple in self.list {
            if frame[apple.x][apple.y] == " " {
                frame[apple.x][apple.y] = "🍎";
            }
        }
    }
}