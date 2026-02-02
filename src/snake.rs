use std::collections::VecDeque;
use std::ops::Add;
use rand::Rng;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn give_new_position(&self) -> Point{
        match self {
            Direction::Up => Point{x: 0, y: -1},
            Direction::Right => Point{x: 1, y: 0},
            Direction::Left => Point{x: -1, y: 0},
            Direction::Down => Point{x: 0, y: 1}
        }
    }
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}



pub struct Snake {
    pub body: VecDeque<Point>,
}

impl Snake {
    pub fn take_one_step(&mut self, direction: Direction, apple_eaten: bool){
        let new_head_position = self.body[0] + direction.give_new_position();
        self.body.push_front(new_head_position);
        if !apple_eaten{
            self.body.pop_back();
        } 
    }
}

pub struct Game {
    pub size_game: i16,
    pub snake: Snake,
    pub direction: Direction,
    pub apple: Point,
    pub running_game: bool,
}

impl Game {
    pub fn take_one_step(&mut self) {
        let apple_eaten = self.check_apple_eaten();
        self.snake.take_one_step(self.direction, apple_eaten);
        self.check_game_is_running();
    }
    
    fn check_game_is_running(&mut self){
        let head_snake = self.snake.body[0];
        let is_too_small = (head_snake.x < 0) || (head_snake.y < 0);
        let is_too_big = (head_snake.x >= self.size_game) || (head_snake.y >= self.size_game);
        if is_too_big || is_too_small {
            self.running_game = false;
        }
    }

    fn check_apple_eaten(&mut self) -> bool {
        if self.apple == self.snake.body[0] {    
            let mut rng = rand::thread_rng();
            self.apple = Point{
                x: rng.gen_range(0..self.size_game), y: rng.gen_range(0..self.size_game)
            };
            true
        } else {
            false
        }
    }
}