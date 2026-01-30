use crate::snake::Snake;
use crate::snake::Point;

pub fn display_game(
    snake: &Snake,
    apple_position: &Point,
){
    let head_pos = snake.body[0];
    let mut string_to_show = String::from("");
    for y in 0..15{
        for x in 0..15 {
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
        string_to_show.push('\n');
    }
    clean_and_display_terminal(string_to_show);
} 


fn clean_and_display_terminal(string_to_show: String) {
    // print!("\x1b[H");
    println!("{}", string_to_show);
}