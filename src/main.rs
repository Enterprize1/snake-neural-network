#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod ring_buffer;
mod neural_network;
mod snake;

use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};

use crate::neural_network::NeuralNetwork;
use crate::snake::{Direction, Snake, TickResult};

const LEARNING_RATE: f64 = 0.001;
const DISCOUNT_FACTOR: f64 = 0.9;

fn main() -> () {
    let mut nn = NeuralNetwork::new(10*10*3, 4);
    // nn.add_hidden_layer(10);
    nn.add_hidden_layer(10, 0);
    // nn.add_hidden_layer(10, 1);

    for j in 0..200000 {
        let mut snake = Snake::new(10, 10);

        let mut score = 0;
        let mut game_steps : Vec<GameStep> = Vec::new();
        loop {
            let snake_state = Array2::from_shape_vec((1, 10 * 10 * 3), snake.to_vec()).unwrap();

            let activations = nn.forward(&snake_state);
            let a = activations.last().unwrap().clone().into_raw_vec();

            let next_step = match a {
                a if a[0] > a[1] && a[0] > a[2] && a[0] > a[3] => Direction::Up,
                a if a[1] > a[0] && a[1] > a[2] && a[1] > a[3] => Direction::Down,
                a if a[2] > a[0] && a[2] > a[1] && a[2] > a[3] => Direction::Left,
                a if a[3] > a[0] && a[3] > a[1] && a[3] > a[2] => Direction::Right,
                _ => Direction::Up,
            };

            let tick_result = snake.tick(next_step);

            let score_change = match tick_result {
                TickResult::GameOver => -10,
                TickResult::Apple => 20,
                _ => 1,
            };

            game_steps.push(GameStep {
                activations,
                score_change,
                snake_state,
                next_step,
            });

            score += score_change;

            if tick_result == TickResult::GameOver {
                break;
            }
        }

        println!("{}", score);

        for game_step in game_steps {
            let q_change = f64::from(game_step.score_change) + DISCOUNT_FACTOR * f64::from(score);
            let mut y = game_step.activations.last().unwrap().clone();

            if game_step.next_step == Direction::Up {
                y[[0, 0]] = q_change;
            } else if game_step.next_step == Direction::Down {
                y[[0, 1]] = q_change;
            } else if game_step.next_step == Direction::Left {
                y[[0, 2]] = q_change;
            } else if game_step.next_step == Direction::Right {
                y[[0, 3]] = q_change;
            }

            nn.backward(game_step.snake_state, y, game_step.activations, LEARNING_RATE);
        }
    }
}

struct GameStep {
    activations: Vec<Array2<f64>>,
    score_change: i32,
    snake_state: ArrayBase<OwnedRepr<f64>, Ix2>,
    next_step: Direction,
}
