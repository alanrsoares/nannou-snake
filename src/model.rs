extern crate nannou;

use nannou::prelude::*;

pub const WINDOW_SIZE: f32 = 512.0;

pub const HALF_WINDOW_SIZE: f32 = WINDOW_SIZE / 2.0;

pub const SQUARE_SIZE: f32 = WINDOW_SIZE / 50.0;

pub const MOVE_SPEED: f32 = SQUARE_SIZE / 5.0;

#[derive(Debug, PartialEq)]
pub enum Status {
    Playing,
    GameOver,
    Paused,
}

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::Playing => "Playing",
            Status::GameOver => "Game Over",
            Status::Paused => "Paused",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Up => vec2(0.0, MOVE_SPEED),
            Direction::Down => vec2(0.0, -MOVE_SPEED),
            Direction::Left => vec2(-MOVE_SPEED, 0.0),
            Direction::Right => vec2(MOVE_SPEED, 0.0),
        }
    }

    pub fn from_key(key: Key) -> Option<Direction> {
        match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Model {
    pub snake: Vec<Point2>,
    pub food: Point2,
    pub direction: Direction,
    pub status: Status,
    pub score: u32,
    pub last_updated: std::time::Instant,
}

impl Model {
    pub fn new() -> Self {
        let snake = (0..6).map(|i| pt2(-SQUARE_SIZE * i as f32, 0.0)).collect();

        let food = pt2(
            random_range(-HALF_WINDOW_SIZE, HALF_WINDOW_SIZE),
            random_range(-HALF_WINDOW_SIZE, HALF_WINDOW_SIZE),
        );

        let direction = Direction::Right;

        Model {
            snake,
            food,
            direction,
            score: 0,
            status: Status::Playing,
            last_updated: std::time::Instant::now(),
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn move_forward(&mut self) {
        self.last_updated = std::time::Instant::now();

        // the head position should be reset to the opoosite side of the window if it goes out of bounds
        let mut head_position = self.snake[0] + self.direction.to_vec2();
        if head_position.x > HALF_WINDOW_SIZE {
            head_position.x = -HALF_WINDOW_SIZE;
        } else if head_position.x < -HALF_WINDOW_SIZE {
            head_position.x = HALF_WINDOW_SIZE;
        } else if head_position.y > HALF_WINDOW_SIZE {
            head_position.y = -HALF_WINDOW_SIZE;
        } else if head_position.y < -HALF_WINDOW_SIZE {
            head_position.y = HALF_WINDOW_SIZE;
        }

        // the snake should die if it collides with itself
        if self.snake.contains(&head_position) {
            self.die();
            return;
        }

        // check if the head collides with the food, not just equal matches
        if head_position.distance(self.food) < SQUARE_SIZE / 2.0 {
            self.grow();
            self.spawn_food();
            self.increment_score();
        }

        self.snake.insert(0, head_position);
        self.snake.pop(); // Maintain the same length by removing the last segment
    }

    fn grow(&mut self) {
        let head_position = self.snake[0] + self.direction.to_vec2();
        self.snake.insert(0, head_position); // Add a new segment without removing the last segment
    }

    fn spawn_food(&mut self) {
        self.food = pt2(
            random_range(-HALF_WINDOW_SIZE, HALF_WINDOW_SIZE),
            random_range(-HALF_WINDOW_SIZE, HALF_WINDOW_SIZE),
        );

        while self.snake.contains(&self.food) {
            self.spawn_food();
        }
    }

    fn increment_score(&mut self) {
        self.score += 1;
    }

    fn die(&mut self) {
        self.status = Status::GameOver;
    }

    fn pause(&mut self) {
        self.status = Status::Paused;
    }

    fn resume(&mut self) {
        self.status = Status::Playing;
    }

    pub fn toggle_pause(&mut self) {
        match self.status {
            Status::Playing => self.pause(),
            Status::Paused => self.resume(),
            _ => (),
        }
    }
}
