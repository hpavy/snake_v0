use std::collections::VecDeque;
use std::ops::Add;


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
    pub fn take_one_step(&mut self, direction: Direction){
        let new_head_position = self.body[0] + direction.give_new_position();
        self.body.push_front(new_head_position);
        self.body.pop_back();
    }
}

pub struct Game {
    pub snake: Snake,
    pub direction: Direction,
    pub apple: Point,
}

impl Game {
    pub fn take_one_step(&mut self) {
        self.snake.take_one_step(self.direction)
    }
}