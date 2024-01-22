use crate::{
    apple::Apple,
    frame::{center, Drawable, Frame, Pixel, X_NUM_COLS, Y_NUM_ROWS},
};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SnakeBit {
    x: u16,
    y: u16,
    direction: Direction,
}

impl SnakeBit {
    fn is_at(&self, x: u16, y: u16) -> bool {
        self.x == x && self.y == y
    }
}

#[derive(Debug, Clone)]
pub struct Snake(Vec<SnakeBit>);

impl Snake {
    pub fn new() -> Self {
        Self(vec![
            SnakeBit {
                x: center().0,
                y: center().1,
                direction: Direction::Up,
            },
            SnakeBit {
                x: center().0,
                y: center().1 + 1,
                direction: Direction::Up,
            },
        ])
    }
    fn body(&self) -> &Vec<SnakeBit> {
        &self.0
    }
    fn body_mut(&mut self) -> &mut Vec<SnakeBit> {
        &mut self.0
    }
    fn body_occupies(&self, x: u16, y: u16) -> bool {
        self.body().iter().any(|snakebit| snakebit.is_at(x, y))
    }
    fn propose_location(x: u16, y: u16, relative_direction: Direction) -> (u16, u16) {
        let mut proposed_x: i32 = x as i32;
        let mut proposed_y: i32 = y as i32;
        match relative_direction {
            Direction::Up => {
                (proposed_x, proposed_y) = (
                    proposed_x,
                    if proposed_y - 1 < 0 {
                        (Y_NUM_ROWS - 1) as i32
                    } else {
                        proposed_y - 1
                    },
                )
            }
            Direction::Down => {
                (proposed_x, proposed_y) = (
                    proposed_x,
                    if proposed_y + 1 >= Y_NUM_ROWS as i32 {
                        0 as i32
                    } else {
                        proposed_y + 1
                    },
                )
            }
            Direction::Left => {
                (proposed_x, proposed_y) = (
                    if proposed_x - 1 < 0 {
                        (X_NUM_COLS - 1) as i32
                    } else {
                        proposed_x - 1
                    },
                    proposed_y,
                )
            }
            Direction::Right => {
                (proposed_x, proposed_y) = (
                    if proposed_x + 1 >= X_NUM_COLS as i32 {
                        0 as i32
                    } else {
                        proposed_x + 1
                    },
                    proposed_y,
                )
            }
        }
        (
            proposed_x.try_into().unwrap(),
            proposed_y.try_into().unwrap(),
        )
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.body_mut()[0].direction = direction;
    }
    fn push_back(&mut self) -> Result<(), ()> {
        let last_snakebit = &self.body()[self.body().len() - 1];
        let last_snakebit_direction = last_snakebit.direction;
        let (proposed_x, proposed_y) = Self::propose_location(
            last_snakebit.x,
            last_snakebit.y,
            last_snakebit.direction.reverse(),
        );
        if !self.body_occupies(proposed_x, proposed_y) {
            self.body_mut().push(SnakeBit {
                x: proposed_x,
                y: proposed_y,
                direction: last_snakebit_direction,
            });
            Ok(())
        } else {
            Err(())
        }
    }
    fn push_front(&mut self) -> Result<(), ()> {
        let first_snakebit = &self.body()[0];
        let (proposed_x, proposed_y) =
            Self::propose_location(first_snakebit.x, first_snakebit.y, first_snakebit.direction);
        let proposed_direction = first_snakebit.direction;
        if !self.body_occupies(proposed_x, proposed_y) {
            self.body_mut().insert(
                0,
                SnakeBit {
                    x: proposed_x,
                    y: proposed_y,
                    direction: proposed_direction,
                },
            );
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn move_forward(&mut self, apple: &mut Apple) -> Result<(), ()> {
        if let Some(_last_snakebit) = self.body_mut().pop() {
            self.push_front()?;
            if self.can_eat_apple(apple) {
                self.push_back()?;
                apple.relocate();
            }
            Ok(())
        } else {
            Err(())
        }
    }
    fn can_eat_apple(&mut self, apple: &mut Apple) -> bool {
        let first_snakebit = &self.body()[0];
        apple.is_at(first_snakebit.x, first_snakebit.y)
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        for snakebit in self.body() {
            frame.put(snakebit.x, snakebit.y, Pixel::SnakeBit);
        }
    }
}
