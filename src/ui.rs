use crate::snake::Snake;
use crate::snake::Point;
use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, cursor::MoveTo};

pub fn display_game(
    size_game: &i16,
    snake: &Snake,
    apple_position: &Point,
){
    let head_pos = snake.body[0];
    let mut string_to_show = String::from("");
    for y in 0..*size_game {
        for x in 0..*size_game {
            let current_pos = Point {x: x, y: y};
            if head_pos == current_pos{
                string_to_show.push_str("\x1b[32mS \x1b[0m");
            } else if *apple_position == current_pos{
                string_to_show.push_str("\x1b[31mA \x1b[0m")
            } else if snake.body.iter().skip(1).any(|&pos| pos == current_pos) {
                string_to_show.push_str("\x1b[32mo \x1b[0m");
            } else{
                string_to_show.push_str("# ");
            }
        }
        string_to_show.push_str("\r\n");
    }
    clean_and_display_terminal(string_to_show);
} 


fn clean_and_display_terminal(string_to_show: String) {
    let mut out = stdout();
    out.execute(MoveTo(0, 0)).unwrap();
    println!("{}", string_to_show);
    out.flush().unwrap();
}