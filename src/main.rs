use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    Result,
};

use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

const WIDTH: u16 = 20;
const HEIGHT: u16 = 10;

struct Snake {
    segments: Vec<(u16, u16)>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            segments: vec![(2, 2)],
        }
    }

    fn move_up(&mut self) {
        let (head_x, mut head_y) = self.segments[0];
        head_y = if head_y == 0 { HEIGHT - 1 } else { head_y - 1 };
        self.segments.insert(0, (head_x, head_y));
        self.segments.pop();
    }

    // Implement move_down, move_left, move_right similarly

    fn draw(&self) -> Result<()> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;

        for &(x, y) in &self.segments {
            execute!(stdout, cursor::MoveTo(x, y), crossterm::style::Print("█"))?;
        }

        stdout.flush()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;

    let mut snake = Snake::new();

    loop {
        snake.draw()?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Up => snake.move_up(),
                    // Implement handling for other directions
                    _ => {}
                }
            }
        }

        thread::sleep(Duration::from_millis(200));
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
