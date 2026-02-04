mod ui;
mod snake;
use std::thread;
use std::time::Duration;
use crate::snake::{Direction, Game};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::cursor::{Hide, Show};
use std::io::stdout;
use crossterm::ExecutableCommand;

fn main() {
    let mut game = Game::new(9);
    
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    print!("\x1b[2J"); 
    while game.running_game {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('z') => game.direction = Direction::Up,
                    KeyCode::Char('s') => game.direction = Direction::Down,
                    KeyCode::Char('q') => game.direction = Direction::Left,
                    KeyCode::Char('d') => game.direction = Direction::Right,
                    KeyCode::Esc => game.running_game = false,
                    _ => ()
                }
            }
        }

        game.take_one_step();

        ui::display_game(&game.size_game, &game.snake, &game.apple);
        thread::sleep(Duration::from_millis(300))
    }
    disable_raw_mode().unwrap();
    stdout().execute(Show).unwrap();
    println!("Game Over your score is: {}", game.snake.body.len() - 2)
}