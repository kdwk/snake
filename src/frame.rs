use std::io::Stdout;

use crossterm::{QueueableCommand, cursor::MoveTo};

use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new() -> Frame {
    let frame = (0..NUM_COLS).map(|_| (0..NUM_ROWS).map(|_| " ").collect::<Vec<&str>>()).collect::<Frame>();
    frame
}

pub fn render(stdout: &mut Stdout, last_frame: &Frame, frame: Frame) -> Frame { // Returns the used frame
    for (x, col) in frame.iter().enumerate() {
        for (y, &char) in col.iter().enumerate() {
            if last_frame[x][y] != char {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", char);
            }
        }
    }
    frame
}

pub trait Drawable {
    fn draw(self, frame: &mut Frame);
}