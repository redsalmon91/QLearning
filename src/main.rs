extern crate rand;

mod agent;
mod model;
mod util;

use agent::q_learning;

const ACTION_COUNT: usize = 2;
const LEFT: i64 = -1;
const RIGHT: i64 = 1;
const NO_REWARD: f64 = 0f64;
const DUPLICATE_PATH_PUNISHMENT: f64 = -1f64;

fn main() {
    let initial_state = 1u64;
    let win_state = 12u64;
    let lost_state = 0u64;
    let win_rewards = 100f64;
    let lost_rewards = -100f64;
    let episode_count = 100;
    let learning_rate = 0.8f64;
    let greedy_factor = 0.8f64;

    start_agent(initial_state, win_state, lost_state, win_rewards, lost_rewards, episode_count, learning_rate, greedy_factor);
}

fn start_agent(
    initial_state: u64,
    win_state: u64,
    lost_state: u64,
    win_rewards: f64,
    lost_rewards: f64,
    episode_count: usize,
    learning_rate: f64,
    greedy_factor: f64) {

    let apply_action_to_state_function = |state: u64, action: usize, state_history: &Vec<u64>| -> (u64, f64) {
        let move_action = if action == 0 { LEFT } else { RIGHT };
        let future_state = (move_action + state as i64) as u64;

        if future_state == win_state {
            return (win_state, win_rewards)
        }

        if future_state == lost_state {
            return (lost_state, lost_rewards)
        }

        let extra_rewards = state_history.iter().filter(|&n| *n == future_state).count() as f64 * DUPLICATE_PATH_PUNISHMENT;
        (future_state, NO_REWARD + extra_rewards)
    };

    q_learning::q_learning(
        episode_count,
        ACTION_COUNT,
        learning_rate,
        greedy_factor,
        initial_state,
        win_state,
        lost_state,
        &apply_action_to_state_function);
}
