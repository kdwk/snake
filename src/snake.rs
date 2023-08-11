use crate::apple::Apples;
use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};
use crate::frame::Frame;

#[derive(Debug, Clone, Copy)]
pub struct SnakeBit {
    pub x: usize,
    pub y: usize,
    direction: Direction,
}

impl PartialEq for SnakeBit {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !(self.x == other.x && self.y == other.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left, Right, Up, Down
}

// pub type Snake = Vec<SnakeBit>;

pub struct Snake {
    body: Vec<SnakeBit>,
}

impl Snake {
    pub fn new() -> Self {
        let body = (0..2).map(|a| SnakeBit{x:10, y:10+a, direction:Direction::Up}).collect();
        Self { body: body }
    }
    fn try_teleport(&self, mut next_x: isize, mut next_y: isize, direction: Direction) -> (isize, isize){
        match direction {
            Direction::Left => if next_x < 0 {next_x = NUM_COLS as isize -1;}
            Direction::Right => if next_x >= NUM_COLS as isize {next_x = 0;}
            Direction::Up => if next_y < 0 {next_y = NUM_ROWS as isize -1;}
            Direction::Down => if next_y >= NUM_ROWS as isize {next_y = 0;}
        }
        (next_x, next_y)
    }
    pub fn move_forward(&mut self, apples: &Apples) -> bool {
        let mut return_val: bool = false;
        let mut next_x = self.body[0].x as isize;
        let mut next_y = self.body[0].y as isize;
        let body_len = self.body.len();
        match self.body[0].direction {
            Direction::Up => next_y -= 1,
            Direction::Down => next_y += 1,
            Direction::Left => next_x -= 1,
            Direction::Right => next_x += 1,
        }
        (next_x, next_y) = self.try_teleport(next_x, next_y, self.body[0].direction);
        if !self.body.iter().any(|&bit| bit.x==next_x as usize && bit.y==next_y as usize) {
            (self.body[body_len-1].x, self.body[body_len-1].y) = (next_x as usize, next_y as usize);
            return_val = true;
        } else {
            return_val = false;
        }
        return_val
    }
    pub fn move_left(&mut self, apples: &Apples) {
        self.body[0].direction = Direction::Left;
        self.move_forward(apples);
    }
    pub fn move_right(&mut self, apples: &Apples) {
        self.body[0].direction = Direction::Right;
        self.move_forward(apples);
    }
    pub fn move_up(&mut self, apples: &Apples) {
        self.body[0].direction = Direction::Up;
        self.move_forward(apples);
    }
    pub fn move_down(&mut self, apples: &Apples) {
        self.body[0].direction = Direction::Down;
        self.move_forward(apples);
    }
}

impl Drawable for Snake {
    fn draw(self, frame: &mut Frame) {
        for bit in self.body {
            frame[bit.x][bit.y] = "█";
        }
    }
}