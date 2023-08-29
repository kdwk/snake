use crate::apple::Apples;
use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};
use crate::frame::Frame;

pub enum DidEatApple {True, False}
pub enum Health {Dead}

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

pub struct Snake {
    body: Vec<SnakeBit>,
}

impl Snake {
    pub fn new() -> Self {
        let body = (0..2).map(|a| SnakeBit{x:10, y:10+a, direction:Direction::Up}).collect();
        Self { body: body }
    }
    fn try_teleport(&self, mut next_x: isize, mut next_y: isize, direction: Direction) -> (usize, usize){
        match direction {
            Direction::Left => if next_x < 0 {next_x = NUM_COLS as isize -1;}
            Direction::Right => if next_x >= NUM_COLS as isize {next_x = 0;}
            Direction::Up => if next_y < 0 {next_y = NUM_ROWS as isize -1;}
            Direction::Down => if next_y >= NUM_ROWS as isize {next_y = 0;}
        }
        (next_x as usize, next_y as usize)
    }
    fn conflicts(&self, proposed_x: usize, proposed_y: usize) -> bool {
        if self.body.iter().any(|&body_bit| body_bit.x==proposed_x && body_bit.y==proposed_y) {
            true
        } else {false}
    }
    pub fn move_forward(&mut self, apples: &mut Apples) -> Result<DidEatApple, Health> {
        let mut next_x_isize = self.body[0].x as isize;
        let mut next_y_isize = self.body[0].y as isize;
        let body_len = self.body.len();
        match self.body[0].direction {
            Direction::Up => next_y_isize -= 1,
            Direction::Down => next_y_isize += 1,
            Direction::Left => next_x_isize -= 1,
            Direction::Right => next_x_isize += 1,
        }
        let (next_x, next_y) = self.try_teleport(next_x_isize, next_y_isize, self.body[0].direction);
        if !self.conflicts(next_x, next_y) {
            let last_bit: &mut SnakeBit = &mut self.body[body_len-1]; // Get the last SnakeBit
            (last_bit.x, last_bit.y) = (next_x, next_y);
            match apples.try_eat(self.body[0].x, self.body[0].y) {
                Ok(()) => {
                    match self.insert_back() {
                        Ok(()) => Ok(DidEatApple::True),
                        Err(health_status) => return Err(health_status)
                    }
                }
                _ => Ok(DidEatApple::False)
            }
        } else {
            Err(Health::Dead)
        }
    }
    pub fn move_left(&mut self) {
        self.body[0].direction = Direction::Left;
    }
    pub fn move_right(&mut self) {
        self.body[0].direction = Direction::Right;
    }
    pub fn move_up(&mut self) {
        self.body[0].direction = Direction::Up;
    }
    pub fn move_down(&mut self) {
        self.body[0].direction = Direction::Down;
    }
    pub fn insert_back(&mut self) -> Result<(), Health> {
        let last_bit: &SnakeBit = &self.body[self.body.len()-1];
        let mut new_bit = SnakeBit {x: last_bit.x, y: last_bit.y, direction: last_bit.direction };
        let mut new_bit_x_isize = new_bit.x as isize;
        let mut new_bit_y_isize = new_bit.y as isize;
        match last_bit.direction {
            Direction::Up => new_bit_y_isize += 1,
            Direction::Down => new_bit_y_isize -= 1,
            Direction::Left => new_bit_x_isize += 1,
            Direction::Right => new_bit_x_isize -= 1,
        }
        (new_bit.x, new_bit.y) = self.try_teleport(new_bit_x_isize, new_bit_y_isize, new_bit.direction);
        if !self.conflicts(new_bit.x, new_bit.y) {
            Ok(())
        } else {
            Err(Health::Dead)
        }
    }
}

impl Drawable for Snake {
    fn draw(&mut self, frame: &mut Frame) {
        for bit in &self.body {
            frame[bit.x][bit.y] = "█";
        }
    }
}