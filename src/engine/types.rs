use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Size {
    #[allow(dead_code)]
    pub fn new(w: i32, h: i32) -> Self {
        Self { w, h }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    #[allow(dead_code)]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x as f32 
            && point.x < (self.x + self.w) as f32
            && point.y >= self.y as f32 
            && point.y < (self.y + self.h) as f32
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    #[allow(dead_code)]
    pub fn to_macroquad(&self) -> macroquad::prelude::Rect {
        macroquad::prelude::Rect::new(
            self.x as f32,
            self.y as f32,
            self.w as f32,
            self.h as f32,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_angle(&self) -> f32 {
        match self {
            Direction::Up => 0.0,
            Direction::Right => 90.0,
            Direction::Down => 180.0,
            Direction::Left => 270.0,
        }
    }
    
    pub fn to_velocity(&self, speed: f32) -> (f32, f32) {
        match self {
            Direction::Up => (0.0, -speed),
            Direction::Down => (0.0, speed),
            Direction::Left => (-speed, 0.0),
            Direction::Right => (speed, 0.0),
        }
    }
}
