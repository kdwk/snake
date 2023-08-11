use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, event::{Event, KeyCode, self}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, cursor::{Show, Hide}, ExecutableCommand};
use crossbeam::{channel, epoch::Pointable};
use snake::{frame::{self, render}, snake::Snake, apple::Apples};
use std::{error::Error, io::{self, Write}, thread, time::{Duration, Instant}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = channel::unbounded();
    let render_thread = thread::spawn(move || {
        let mut last_frame = frame::new();
        let mut stdout = io::stdout();
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.flush().unwrap();
        loop {
            let current_frame = match render_rx.recv() {
                Ok(returned_frame) => returned_frame,
                Err(_) => break
            };
            last_frame = render(&mut stdout, &last_frame, current_frame);
        }
    });

    let snake = Snake::new();
    let apples = Apples::init();
    let score: usize = 0;

    'gameloop: loop {
        todo!()
    }

    Ok(())
}
