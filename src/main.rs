mod ui;
mod snake;
use crate::snake::Snake;
use std::collections::VecDeque; 

fn main() {
    let apple = (9, 4);
    let snake = Snake{
        body:VecDeque::from(vec![(4, 4), (4, 5), (4, 6), (5, 6)])
    };

    ui::display_game(&snake, &apple);
}