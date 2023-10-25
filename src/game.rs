pub struct Block {
    pub y: i32,
    pub x: i32,
    pub direction: Direction,
}

impl Block {
    pub fn is_collision(&self, snake: &Snake) -> bool {
        snake.iter().any(|b| b.x == self.x && b.y == self.y)
    }

    pub fn move_forward(&self, direction: Direction) -> Block {
        match direction {
            Direction::Up => Block {
                y: self.y - 1,
                x: self.x,
                direction,
            },
            Direction::Down => Block {
                y: self.y + 1,
                x: self.x,
                direction,
            },
            Direction::Right => Block {
                y: self.y,
                x: self.x + 1,
                direction,
            },
            Direction::Left => Block {
                y: self.y,
                x: self.x - 1,
                direction,
            },
        }
    }
}

pub enum GameStatus {
    GameOver,
    Paused,
    Playing,
    New,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_legal_move(&self, state: &State) -> bool {
        state.game_status == GameStatus::Playing && self != &self.opposite()
    }
}

pub type Snake = Vec<Block>;

impl Snake {
    pub fn hd(&self) -> &Block {
        &self[0]
    }

    pub fn tl(&self) -> Snake {
        match self.len() {
            0 | 1 => Vec::new(),
            2 => vec![self[1]],
            _ => self[1..self.len() - 1].to_vec(),
        }
    }

    pub fn last(&self) -> &Block {
        &self[self.len() - 1]
    }

    pub fn direction(&self) -> Direction {
        self.hd().direction
    }

    pub fn drop_last(&mut self) {
        self.pop();
    }

    pub fn move_forward(&mut self, direction: Direction) {
        let new_head = self.hd().move_forward(direction);
        self.insert(0, new_head);
        self.drop_last();
    }

    pub fn grow(&mut self) {
        let last_block = self.last();
        let new_block = Block {
            direction: last_block.direction,
            ..last_block.move_forward(last_block.direction)
        };
        self.push(new_block);
    }

    pub fn has_eaten(&self, food: &Block) -> bool {
        self.hd().is_collision(&vec![food.clone()])
    }

    pub fn has_collided_with_self(&self) -> bool {
        self.hd().is_collision(&self.tl())
    }

    pub fn detect_collision(&self, food: &Block) -> Option<Collision> {
        if self.has_eaten(food) {
            Some(Collision::Food)
        } else if self.has_collided_with_self() {
            Some(Collision::Tail)
        } else {
            None
        }
    }

    pub fn spawn_food(&self) -> Block {
        loop {
            let new_food = Block {
                x: random_in_range(0, Config::tiles), // Assuming you have a random_in_range function and Config struct
                y: random_in_range(0, Config::tiles),
                direction: Direction::Right,
            };
            if !new_food.is_collision(self) {
                return new_food;
            }
        }
    }
}

pub enum Action {
    TogglePause,
    Start,
    Reset,
    Move(Direction),
    AddMoveToQueue(Direction),
}

pub struct State {
    pub snake: Snake,
    pub game_status: GameStatus,
    pub score: i32,
    pub high_score: i32,
    pub moves: Vec<Direction>,
    pub food: Block,
    pub last_moved_timestamp: f64,
}

pub enum Collision {
    Food,
    Tail,
}

pub const INITIAL_SNAKE: [Block; 6] = (1..=6)
    .rev()
    .map(|x| Block {
        y: 1,
        x,
        direction: Direction::Right,
    })
    .collect::<Vec<Block>>()
    .try_into()
    .expect("Wrong number of elements"); // This converts the Vec into an array.

pub const INITIAL_STATE: State = State {
    snake: INITIAL_SNAKE.to_vec(),
    game_status: GameStatus::New,
    score: 0,
    high_score: 0,
    moves: Vec::new(),
    food: spawn_food(&INITIAL_SNAKE.to_vec()),
    last_moved_timestamp: 0.0,
};

pub struct State {
    snake: Snake,
    game_status: GameStatus,
    score: i32,
    high_score: i32,
    moves: Vec<Direction>,
    food: Block,
    last_moved_timestamp: f64,
}

impl State {
    fn reduce_move(&mut self, direction: Direction) {
        let time_elapsed = Helpers::time_elapsed(self.last_moved_timestamp); // Assuming you have a Helpers struct/method

        if time_elapsed >= Config::speed {
            self.last_moved_timestamp = Helpers::now(); // Assuming you have a now method in Helpers
            self.snake.move_forward(direction); // Assuming you have a move method in Snake
            if self.moves.len() > 0 {
                self.moves.remove(0);
            }

            match self.snake.detect_collision(&self.food) {
                // Assuming you have a detect_collision method in Snake
                None => {}
                Some(Collision::Tail) => {
                    self.game_status = GameStatus::GameOver;
                }
                Some(Collision::Food) => {
                    self.score += 1;
                    self.food = self.snake.spawn_food();
                    self.snake.grow(); // Assuming you have a grow method in Snake
                }
            }
        }
    }

    fn reduce_toggle_pause(&mut self) {
        match self.game_status {
            GameStatus::New => self.game_status = GameStatus::Playing,
            GameStatus::Playing => self.game_status = GameStatus::Paused,
            GameStatus::Paused => self.game_status = GameStatus::Playing,
            GameStatus::GameOver => {
                *self = State {
                    game_status: GameStatus::New,
                    ..INITIAL_STATE
                };
            }
        }
    }

    pub fn reduce(&mut self, action: Action) {
        match action {
            Action::Reset => *self = INITIAL_STATE,
            Action::Move(direction) => self.reduce_move(direction),
            Action::TogglePause => self.reduce_toggle_pause(),
            Action::AddMoveToQueue(direction) => {
                if direction.is_legal_move(self) {
                    self.moves.push(direction);
                }
            }
            _ => {}
        }
    }
}
