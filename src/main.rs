mod ui;
mod snake;
use std::thread;
use std::time::Duration;
use crate::snake::Snake;
use crate::snake::Point;
use crate::snake::Direction;
use std::collections::VecDeque; 
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::cursor::{Hide, Show};
use std::io::stdout;
use crossterm::ExecutableCommand;

fn main() {
    let apple = Point {x: 9, y: 4};
    let mut snake = Snake{
        body:VecDeque::from([
            Point{x: 4, y: 4},
            Point{x: 4, y: 5},
            ]),
        direction: Direction::Up
    };
    
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    print!("\x1b[2J"); 
    loop {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('z') => snake.direction = Direction::Up,
                    KeyCode::Char('s') => snake.direction = Direction::Down,
                    KeyCode::Char('q') => snake.direction = Direction::Left,
                    KeyCode::Char('d') => snake.direction = Direction::Right,
                    KeyCode::Esc => break,
                    _ => ()
                }
            }
        }

        snake.take_one_step();

        ui::display_game(&snake, &apple);
        thread::sleep(Duration::from_millis(700))
    }
    disable_raw_mode().unwrap();
    stdout().execute(Show).unwrap();
}