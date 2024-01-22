use std::{
    error::Error,
    io::{Stdout, Write},
};

use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};

pub const X_NUM_COLS: u16 = 30;
pub const Y_NUM_ROWS: u16 = 30;

pub enum Pixel {
    Blank,
    SnakeBit,
    Apple,
}

impl From<&Pixel> for String {
    fn from(value: &Pixel) -> Self {
        match value {
            Pixel::Blank => "  ".into(),
            Pixel::Apple => "🍎".into(),
            Pixel::SnakeBit => "██".into(),
        }
    }
}

pub struct Frame(Vec<Vec<Pixel>>);

impl Frame {
    pub fn new() -> Frame {
        Frame(
            (0..Y_NUM_ROWS)
                .map(|_| {
                    (0..X_NUM_COLS)
                        .map(|_| Pixel::Blank)
                        .collect::<Vec<Pixel>>()
                })
                .collect::<Vec<Vec<Pixel>>>(),
        )
    }
    fn content(&self) -> &Vec<Vec<Pixel>> {
        &self.0
    }
    fn content_mut(&mut self) -> &mut Vec<Vec<Pixel>> {
        &mut self.0
    }
    pub fn put(&mut self, x: u16, y: u16, pixel: Pixel) {
        self.content_mut()[y as usize][x as usize] = pixel;
    }
    pub fn render(self, out: &mut Stdout) -> Result<(), Box<dyn Error>> {
        for (y_coordinate, row) in self.content().iter().enumerate() {
            for (x_coordinate, pixel) in row.iter().enumerate() {
                out.queue(MoveTo((x_coordinate * 2) as u16, y_coordinate as u16))?
                    .queue(Print(String::from(pixel)))?;
            }
        }
        out.flush()?;
        Ok(())
    }
}

pub fn center() -> (u16, u16) {
    (X_NUM_COLS / 2, Y_NUM_ROWS / 2)
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
