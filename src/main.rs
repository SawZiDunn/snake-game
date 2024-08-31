#![allow(unused_must_use)]
#![allow(dead_code)]

use std::{
    io::{stdout, Write},
    process,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use rand::{thread_rng, Rng};

const WIDTH: u16 = 50;
const HEIGHT: u16 = 30;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Item {
    figure: char,
    color: Color,
    x: u16,
    y: u16,
}

impl Item {
    fn new(figure: char, color: Color, x: u16, y: u16) -> Self {
        Item {
            figure,
            color,
            x,
            y,
        }
    }

    fn print_item(&self) {
        queue!(
            stdout(),
            cursor::MoveTo(self.x, self.y),
            style::SetForegroundColor(self.color),
            Print(self.figure)
        )
        .unwrap();
        stdout().flush().unwrap();
    }
}

struct Bomb {
    item: Item,
    visible: bool,
    timeout: u64,
}

impl Bomb {
    fn new_bomb(figure: char, color: Color, x: u16, y: u16, timeout: u64) -> Self {
        Bomb {
            item: Item::new(figure, color, x, y),
            visible: true,
            timeout,
        }
    }

    fn print_bomb(&self) {
        if self.visible {
            self.item.print_item();
        }
    }

    fn hide_bomb(&mut self) {
        self.visible = false;
    }

    fn reset_bomb(&mut self, x: u16, y: u16) {
        self.item.x = x;
        self.item.y = y;
        self.visible = true;
    }
}

#[derive(Clone, PartialEq)]
struct Snake {
    body: Vec<Item>,
    direction: Direction,
}

impl Snake {
    fn new(head: Item) -> Self {
        Snake {
            body: vec![head],
            direction: Direction::Up,
        }
    }

    fn get_head(&self) -> &Item {
        self.body.first().unwrap()
    }

    fn print_snake(&self) {
        for item in self.body.iter() {
            item.print_item();
        }
    }

    fn move_snake(&mut self, will_grow: bool) {
        let head = self.get_head();
        let (head_x, head_y) = match self.direction {
            Direction::Up => match head.y == 0 {
                true => (head.x, HEIGHT - 1),
                false => (head.x, head.y - 1),
            },
            Direction::Down => match head.y == HEIGHT {
                true => (head.x, 1),
                false => (head.x, head.y + 1),
            },
            Direction::Left => match head.x == 0 {
                true => (WIDTH - 1, head.y),
                false => (head.x - 1, head.y),
            },
            Direction::Right => match head.x == WIDTH {
                true => (1, head.y),
                false => (head.x + 1, head.y),
            },
        };
        let new_head = Item::new('X', Color::Blue, head_x, head_y);

        if !will_grow {
            self.body.pop();
        }

        self.body.insert(0, new_head);
    }

    fn change_direction(&mut self, key: KeyCode) {
        let new_direction = match key {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => self.direction,
        };

        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
        }
    }
}

fn check_collision(a: &Item, b: &Item) -> bool {
    a.x == b.x && a.y == b.y
}

fn check_hit_wall(head: &Item) -> bool {
    head.x >= WIDTH || head.x < 1 || head.y < 1 || head.y >= HEIGHT
}

fn initialize_screen() {
    let mut stdout = stdout();
    queue!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    enable_raw_mode();
}

fn terminate_screen() {
    queue!(stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    disable_raw_mode();
}

fn clear_screen() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn print_border() {
    for i in 0..=WIDTH {
        queue!(
            stdout(),
            cursor::MoveTo(i, 0),
            SetForegroundColor(Color::Yellow),
            Print("="),
            cursor::MoveTo(i, HEIGHT),
            Print("=")
        )
        .unwrap();
    }

    for i in 0..=HEIGHT {
        queue!(
            stdout(),
            cursor::MoveTo(0, i),
            Print("="),
            cursor::MoveTo(WIDTH, i),
            Print("=")
        )
        .unwrap();
    }

    stdout().flush().unwrap();
}

fn print_status(hp: i32) {
    queue!(
        stdout(),
        cursor::MoveTo(WIDTH + 5, HEIGHT - 5),
        SetForegroundColor(Color::Red),
        Print(format!("HP: {}", hp)),
    )
    .unwrap();
    stdout().flush().unwrap()
}

fn win() {
    queue!(
        stdout(),
        cursor::MoveTo((WIDTH / 2) - 5, HEIGHT / 2 - 1),
        SetForegroundColor(Color::Green),
        Print("You Win!"),
        cursor::MoveTo((WIDTH / 2) - 6, HEIGHT / 2),
        SetForegroundColor(Color::White),
        Print("r - Restart"),
        cursor::MoveTo((WIDTH / 2) - 5, (HEIGHT / 2) + 1),
        Print("Esc - Quit")
    )
    .unwrap();
    stdout().flush().unwrap();
}

fn print_game_over_screen() {
    queue!(
        stdout(),
        cursor::MoveTo((WIDTH / 2) - 5, HEIGHT / 2 - 1),
        SetForegroundColor(Color::Red),
        Print("GAME OVER"),
        cursor::MoveTo((WIDTH / 2) - 6, HEIGHT / 2),
        SetForegroundColor(Color::White),
        Print("r - Restart"),
        cursor::MoveTo((WIDTH / 2) - 5, (HEIGHT / 2) + 1),
        Print("Esc - Quit")
    )
    .unwrap();
    stdout().flush().unwrap();
}

fn main() {
    let (columns, rows) = terminal::size().unwrap();

    if rows < 30 || columns < 15 {
        println!("Minumum terminal size required is 30x15! Please try again!");
        process::exit(0);
    }

    initialize_screen();

    let mut food = Item::new('$', Color::Blue, WIDTH / 2, HEIGHT / 2);
    let head = Item::new('X', Color::Red, WIDTH / 2, HEIGHT / 2 + 6);
    let mut snake = Snake::new(head);
    let mut rng = thread_rng();

    let mut bomb = Bomb::new_bomb(
        'O',
        Color::Red,
        rng.gen_range(1..WIDTH),
        rng.gen_range(1..HEIGHT),
        7000,
    );

    let mut last_bomb_time = std::time::Instant::now(); // getting current time

    loop {
        clear_screen();
        food.print_item();
        snake.print_snake();
        print_border();
        print_status(snake.body.len() as i32);
        bomb.print_bomb();

        if bomb.visible && last_bomb_time.elapsed().as_millis() > bomb.timeout as u128 {
            bomb.hide_bomb();
        }

        // Randomly reappear the bomb after a certain duration
        if !bomb.visible && rng.gen_bool(0.01) {
            bomb.reset_bomb(rng.gen_range(1..WIDTH), rng.gen_range(1..HEIGHT));
            last_bomb_time = std::time::Instant::now();
        }

        // adjusting the speed of timeout based on the snake's body length
        let timeout = match snake.direction {
            Direction::Left | Direction::Right => 100 - snake.body.len(),
            Direction::Up | Direction::Down => 150 - snake.body.len(),
        };

        if event::poll(Duration::from_millis(timeout as u64)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } = key
                {
                    process::exit(1)
                }

                snake.change_direction(key.code);
            }
        }

        let head = snake.body[0].clone();
        let will_grow = check_collision(&head, &food);
        let hit_wall = check_hit_wall(&head);
        let hit_bomb = bomb.visible && check_collision(&head, &bomb.item);

        if will_grow {
            loop {
                food.x = rng.gen_range(1..WIDTH);
                food.y = rng.gen_range(1..HEIGHT);
                if !snake.body.iter().any(|item| check_collision(&item, &food)) {
                    break;
                }
            }

            if snake.body.len() + 1 == 10 {
                snake.move_snake(true); // Grow the snake one last time before winning
                win(); // Display the "You Win!" message
                break; // Exit the game loop
            }
        } else if hit_wall && snake.body.len() < 10 {
            break;
        }

        if hit_bomb {
            if snake.body.len() > 1 {
                // Decrease snake length by 1
                snake.body.pop();
                bomb.hide_bomb();
            } else {
                break;
            }
        }

        let collide = snake // check collision with the snake itself
            .body
            .iter()
            .enumerate()
            .any(|(i, cell)| i != 0 && check_collision(&head, cell));

        if collide {
            break;
        }

        snake.move_snake(will_grow);
    }

    if snake.body.len() >= 10 {
        // defining win state
        win();
    } else {
        print_game_over_screen();
    }

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => {
                    terminate_screen();
                    process::exit(0);
                }
                KeyCode::Char('r') => main(),
                _ => (),
            }
        }
    }
}
