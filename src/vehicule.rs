use std::time::Instant;

const CAR_WIDTH: u32 = 25;
const CAR_HEIGHT: u32 = 30;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up, Down, Left, Right,
}

#[derive(Clone, Copy)]
pub struct Vehicule {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub speed: i32,
    pub distance: i32,
    pub timer: Instant,
    pub states: bool,
    pub frame_count: u32,
    pub angle: f64,
    pub turning: bool, 
}

impl Vehicule {
    pub fn new(x: i32, y: i32, direction: Direction, angle: f64) -> Self {
        Vehicule {
            x, y, direction, speed: 3, distance: 0,
            timer: Instant::now(),
            states: true, frame_count: 0, angle, turning: false,
        }
    }

    pub fn collitions(&self, other: &Vehicule, safe_distance: i32) -> bool {
        match self.direction {
            Direction::Down => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.y >= self.y { return dy <= safe_distance && dx < (CAR_WIDTH as i32); }
            }
            Direction::Up => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.y >= other.y { return dy <= safe_distance && dx < (CAR_WIDTH as i32); }
            }
            Direction::Left => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.x >= other.x { return dx <= safe_distance && dy < (CAR_HEIGHT as i32); }
            }
            Direction::Right => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.x >= self.x { return dx <= safe_distance && dy < (CAR_HEIGHT as i32); }
            }
        }
        false
    }

    pub fn should_turning(&self) -> Option<Direction> {
        if !self.turning { return None; }
        match self.direction {
            Direction::Up => {
                if self.y <= 355 && self.x == 410 { Some(Direction::Left) }
                else if self.y <= 490 && self.x == 500 { Some(Direction::Right) }
                else { None }
            }
            Direction::Down => {
                if self.y >= 405 && self.x == 365 { Some(Direction::Right) }
                else if self.y >= 270 && self.x == 275 { Some(Direction::Left) }
                else { None }
            }
            Direction::Left => {
                if self.y == 270 && self.x <= 500 { Some(Direction::Up) }
                else if self.y == 360 && self.x <= 365 { Some(Direction::Down) }
                else { None }
            }
            Direction::Right => {
                if self.y == 400 && self.x >= 405 { Some(Direction::Up) }
                else if self.y == 490 && self.x >= 275 { Some(Direction::Down) }
                else { None }
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(new_dir) = self.should_turning() {
            self.direction = new_dir;
            self.angle = match new_dir {
                Direction::Up => 0.0,
                Direction::Down => 180.0,
                Direction::Left => -90.0,
                Direction::Right => 90.0,
            };
            self.turning = false;
            self.speed = 0;
        }
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
        }
        self.distance += self.speed;
    }

  
}