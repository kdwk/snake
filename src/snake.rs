use crate::frame::Drawable;

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

enum Edge {
    LeftEdge, RightEdge, TopEdge, BottomEdge, No
}

enum Direction {
    Left, Right, Up, Down, Teleport(Edge),
    No // ONLY use for illustrating a fictional SnakeBit for comparison
}

// pub type Snake = Vec<SnakeBit>;

pub struct Snake {
    body: Vec<SnakeBit>,
    length: usize,
}

impl Snake {
    pub fn init() -> Self {
        let body = (0..2).map(|a| SnakeBit{x:10, y:10+a, direction:Direction::Up}).collect();
        Self { body: body, length: 3 }
    }
    pub fn move_forward(&mut self) -> Result<(), ()> {
        // match self.body[0].direction {
        //     Direction::Up => {
        //         let next_y = if self.body[0].y == 0
        //         if !self.body.contains(&SnakeBit { x: self.body[0].x, y: self.body[0].y-1, direction:Direction::No }) {

        //             Ok(())
        //         } else {
        //             Err(())
        //         }
        //     }
        // }
        todo!()
    }
    pub fn move_left(self) {

    }
}

impl Drawable for Snake {
    fn draw(self, frame: &mut crate::frame::Frame) {
        for bit in self.body {
            frame[bit.x][bit.y] = "█";
        }
    }
}