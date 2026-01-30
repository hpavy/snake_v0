mod ui;
mod snake;
use crate::snake::Snake;
use crate::snake::Point;
use std::collections::VecDeque; 

fn main() {
    let apple = Point {x: 9, y: 4};
    let snake = Snake{
        body:VecDeque::from([
            Point{x: 4, y: 4},
            Point{x: 4, y: 5},
            ])
    };

    ui::display_game(&snake, &apple);
}