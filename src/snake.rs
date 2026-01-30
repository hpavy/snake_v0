use std::collections::VecDeque;
use std::ops::Add;


// #[derive(Debug, PartialEq, Copy, Clone)]
// pub enum Direction {
//     Up,
//     Left,
//     Right,
//     Down,
// }



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
    // pub direction:Direction
}

// impl Snake {
//     fn take_one_step(&self){
        
//     }
// }