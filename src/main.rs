mod ui;
mod snake;
use std::thread;
use std::time::Duration;
use crate::snake::{Snake, Point, Direction, Game};
use std::collections::VecDeque; 
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::cursor::{Hide, Show};
use std::io::stdout;
use crossterm::ExecutableCommand;

fn main() {
    let snake = Snake{
        body:VecDeque::from([
            Point{x: 4, y: 4},
            Point{x: 4, y: 5},
            ]),
        };
    let mut game = Game{
        snake: snake,
        direction: Direction::Up,
        apple: Point {x: 9, y: 4} 
    };
    
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    print!("\x1b[2J"); 
    loop {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('z') => game.direction = Direction::Up,
                    KeyCode::Char('s') => game.direction = Direction::Down,
                    KeyCode::Char('q') => game.direction = Direction::Left,
                    KeyCode::Char('d') => game.direction = Direction::Right,
                    KeyCode::Esc => break,
                    _ => ()
                }
            }
        }

        game.take_one_step();

        ui::display_game(&game.snake, &game.apple);
        thread::sleep(Duration::from_millis(700))
    }
    disable_raw_mode().unwrap();
    stdout().execute(Show).unwrap();
}