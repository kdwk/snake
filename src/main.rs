use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, event::{Event, KeyCode, self}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, cursor::{Show, Hide}, ExecutableCommand};
use crossbeam::{channel, epoch::Pointable};
use snake::{frame::{self, render, Frame, Drawable}, snake::{Snake, DidEatApple, Health}, apple::Apples};
use std::{error::Error, io::{self, Write}, thread, time::{Duration, Instant}, vec};

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

    let mut snake = Snake::new();
    let mut apples = Apples::init();
    let mut score: usize = 0;

    'gameloop: loop {
        let mut current_frame = Frame::new();
        while event::poll(Duration::default())? {
            let event = event::read()?;
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break 'gameloop,
                    KeyCode::Left => snake.move_left(),
                    KeyCode::Right => snake.move_right(),
                    KeyCode::Down => snake.move_down(),
                    KeyCode::Up => snake.move_up(),
                    _ => {}
                },
                _ => {} // Ignore if not key event
            }
        }

        match snake.move_forward(&mut apples) {
            Ok(did_eat_apple) => match did_eat_apple {
                DidEatApple::True => score += 1,
                DidEatApple::False => {},
            },
            Err(health_status) => match health_status {
                Health::Dead => break 'gameloop,
            },
        }

        // let drawables: Vec<&dyn Drawable> = vec![&snake, &apples];
        // for drawable in drawables {
        //     drawable.draw(&mut current_frame);
        // }
        snake.draw(&mut current_frame);
        apples.draw(&mut current_frame);

        let _ = render_tx.send(current_frame);
    }

    Ok(())
}
