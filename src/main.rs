extern crate rand;

mod agent;
mod model;
mod util;

use std::env;
use agent::q_learning;

const ACTION_COUNT: usize = 2;
const LEFT: i64 = -1;
const RIGHT: i64 = 1;

const NO_REWARD: f64 = 0f64;
const WIN_REWARDS: f64 = 100f64;
const LOST_REWARDS: f64 = -100f64;
const DUPLICATE_PATH_PUNISHMENT: f64 = -1f64;

const ARGV_COUNT: usize = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != ARGV_COUNT {
        println!("Expected {} arguments, found {}", ARGV_COUNT, args.len());
        println!("Usage:\n\tq_learning <map-length> <initial-index> \
            <training-episode-count> <learning-rate> <greedy-factor>\n\n");
        return;
    }

    let state_count = args[1].parse::<u64>().unwrap();
    let initial_state = args[2].parse::<u64>().unwrap();
    let episode_count = args[3].parse::<usize>().unwrap();
    let learning_rate = args[4].parse::<f64>().unwrap();
    let greedy_factor = args[5].parse::<f64>().unwrap();

    start_agent(state_count, initial_state, episode_count, learning_rate, greedy_factor);
}

fn start_agent(
    state_count: u64,
    initial_state: u64,
    episode_count: usize,
    learning_rate: f64,
    greedy_factor: f64
) {
    let apply_action_to_state_function = |state: u64, action: usize, state_history: &Vec<u64>| -> (u64, f64) {
        let move_action = if action == 0 { LEFT } else { RIGHT };
        let future_state = (move_action + state as i64) as u64;

        if future_state == state_count - 1 {
            return (state_count - 1, WIN_REWARDS)
        }

        if future_state == 0 {
            return (0, LOST_REWARDS)
        }

        let extra_rewards = state_history.iter().filter(|&n| *n == future_state).count() as f64 * DUPLICATE_PATH_PUNISHMENT;
        (future_state, NO_REWARD + extra_rewards)
    };

    q_learning::q_learning(
        episode_count,
        ACTION_COUNT,
        learning_rate,
        greedy_factor,
        state_count,
        initial_state,
        &apply_action_to_state_function);
}
