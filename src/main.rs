use crossterm::{event::{Event, KeyCode, self}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Show, Hide}, ExecutableCommand};
use crossbeam::{channel};
use std::{error::Error, io, thread, time::{Duration, Instant}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    

    Ok(())
}
