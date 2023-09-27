use rand::Rng;
use crate::ring_buffer::RingBuffer;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GameState {
    Running,
    GameOver,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TickResult {
    Ok,
    GameOver,
    Apple,
}


#[derive(Debug, Clone)]
pub struct Snake {
    pub width: usize,
    pub height: usize,
    apple: Coord,
    snake_path: RingBuffer<Coord>,
    last_direction: Direction,
    state: GameState,
}

impl Snake {
    pub fn new(width: usize, height: usize) -> Self {
        let half_width = width / 2;
        let half_height = height / 2;

        // init new snake_path vector starting at half_width
        let mut snake_path = RingBuffer::new(width * height, 3);
        for y in (half_height - 1)..(half_height + 2) {
            snake_path.push(Coord {
                x: half_width,
                y,
            });
        }

        let apple = Self::new_apple(&snake_path, width, height);

        Self {
            width,
            height,
            apple,
            snake_path,
            last_direction: Direction::Down,
            state: GameState::Running,
        }
    }

    fn new_apple(snake_path: &RingBuffer<Coord>, width: usize, height: usize) -> Coord {
        loop {
            let apple = Coord {
                x: rand::thread_rng().gen_range(0..width),
                y: rand::thread_rng().gen_range(0..height),
            };


            if !snake_path.contains(&apple) {
                return apple;
            }
        }
    }
    pub fn tick(&mut self, direction: Direction) -> TickResult {
        let mut result = TickResult::Ok;

        let first = self.snake_path.first().unwrap();

        if direction == Direction::Up && first.y <= 0 || direction == Direction::Left && first.x <= 0 {
            self.state = GameState::GameOver;
            return TickResult::GameOver;
        }

        let next_coord = match direction {
            Direction::Up => {
                Coord {
                    x: first.x,
                    y: first.y - 1,
                }
            },
            Direction::Down => {
                Coord {
                    x: first.x,
                    y: first.y + 1,
                }
            },
            Direction::Left => {
                Coord {
                    x: first.x - 1,
                    y: first.y,
                }
            },
            Direction::Right => {
                Coord {
                    x: first.x + 1,
                    y: first.y,
                }
            },
        };

        self.last_direction = direction;

        if next_coord.x >= self.width || next_coord.y >= self.height {
            self.state = GameState::GameOver;
            return TickResult::GameOver;
        }

        if self.snake_path.contains(&next_coord) {
            self.state = GameState::GameOver;
            return TickResult::GameOver;
        }

        if next_coord == self.apple {
            self.apple = Self::new_apple(&self.snake_path, self.width, self.height);

            self.snake_path.extend(self.snake_path.last().unwrap().clone());
            self.snake_path.extend(self.snake_path.last().unwrap().clone());
            self.snake_path.extend(self.snake_path.last().unwrap().clone());

            result = TickResult::Apple;
        }

        self.snake_path.push(next_coord);

        result
    }

    pub fn to_vec(&self) -> Vec<f64> {
        let mut v : Vec<f64> = Vec::with_capacity(self.width * self.height * 3);

        // Iterate over coordinates
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord { x, y };

                // Check if the coordinate is part of the snake_path
                if self.snake_path.contains(&coord) {
                    v.push(1f64);
                } else {
                    v.push(0f64);
                }

                // Check if the coordinate is the apple
                if coord == self.apple {
                    v.push(1f64);
                } else {
                    v.push(0f64);
                }

                // Check if the coordinate is the head of the path
                if coord == *self.snake_path.first().unwrap() {
                    v.push(1f64);
                } else {
                    v.push(0f64);
                }
            }
        }

        v
    }
}
