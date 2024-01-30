mod apple;
mod frame;
mod snake;

use std::{
    error::Error,
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};

use apple::Apple;
use crossbeam::channel;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    style::{Color, SetBackgroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use frame::{Drawable, Frame};
use snake::Snake;

use crate::snake::Direction;

#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Into<u128> for Difficulty {
    fn into(self) -> u128 {
        match self {
            Difficulty::Easy => 400,
            Difficulty::Medium => 300,
            Difficulty::Hard => 200,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    out.execute(EnterAlternateScreen)?.execute(Hide)?;
    let (render_sender, render_receiver) = channel::unbounded::<Frame>();
    let render_thread = thread::spawn(move || {
        let mut out = stdout();
        out.queue(Clear(ClearType::All))
            .unwrap()
            .queue(SetBackgroundColor(Color::Blue))
            .unwrap()
            .flush()
            .unwrap();
        if let Err(error) = Frame::new().render(&mut out) {
            eprintln!("{}", error.to_string());
            return;
        }
        loop {
            if let Ok(frame) = render_receiver.recv() {
                if let Err(_error) = frame.render(&mut out) {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            } else {
                break;
            }
        }
    });

    let mut snake = Snake::new();
    let mut apple = Apple::new();
    let mut instant = Instant::now();
    let mut difficulty = Difficulty::Easy;
    let mut difficulty_timer = Instant::now();
    'gameloop: loop {
        let mut frame = Frame::new();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    KeyCode::Up => snake.set_direction(Direction::Up),
                    KeyCode::Down => snake.set_direction(Direction::Down),
                    KeyCode::Left => snake.set_direction(Direction::Left),
                    KeyCode::Right => snake.set_direction(Direction::Right),
                    KeyCode::Char(' ') => {
                        if let Err(_) = event::read() {
                            break 'gameloop;
                        }
                    }
                    _ => {}
                }
            }
        }
        if instant.elapsed().as_millis() >= difficulty.into() {
            if let Err(_) = snake.move_forward(&mut apple) {
                break 'gameloop;
            }
            instant = Instant::now();
        }
        if difficulty_timer.elapsed().as_secs() >= 30 {
            match difficulty {
                Difficulty::Easy => difficulty = Difficulty::Medium,
                Difficulty::Medium => difficulty = Difficulty::Hard,
                _ => (),
            }
            difficulty_timer = Instant::now();
        }
        snake.draw(&mut frame);
        apple.draw(&mut frame);
        frame.render(&mut out)?;
        thread::sleep(Duration::from_millis(5));
    }
    drop(render_sender);
    render_thread.join().expect("Could not join render_thread");
    out.queue(Show)?.queue(LeaveAlternateScreen)?.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}
