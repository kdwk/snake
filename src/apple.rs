use crate::{frame::{Drawable, Frame}, NUM_APPLES, NUM_ROWS, NUM_COLS};
use fastrand as random;

#[derive(Debug, Clone, Copy)]
struct Apple {
    pub x: usize,
    pub y: usize
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !(self.x == other.x && self.y == other.y)
    }
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

impl Apples {
    pub fn init() -> Self {
        let mut list: Vec<Apple> = Vec::new();
        for _ in 0..NUM_APPLES {
            let mut x = random::usize(0..NUM_COLS);
            let mut y = random::usize(0..NUM_ROWS);
            while list.contains(&Apple { x: x, y: y }){
                x = random::usize(0..NUM_COLS);
                y = random::usize(0..NUM_ROWS);
            }
            list.push(Apple::new(x, y));
        }
        Apples { list: list }
    }
    pub fn try_eat(&mut self, x: usize, y: usize) -> Result<(), ()> {
        let old_len = self.list.len();
        self.list.retain(|&apple| apple == Apple::new(x, y));
        if self.list.len() < old_len {
            Ok(())
        } else {
            Err(())
        }
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