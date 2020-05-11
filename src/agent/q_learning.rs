use std::{thread, time};
use util::io_utils;

use model::def::{Action, Dimension, Reward, State, LOSS_PER_TIME_UNIT, PUNISHMENT, REWARD};
use model::map::Map;
use model::q_table::QTable;

const LEFT: Action = 0;
const RIGHT: Action = 1;
const UP: Action = 2;
const DOWN: Action = 3;

const SUCCEED_MSG: &str = "✓";
const FAIL_MSG: &str = "❌";

pub struct Agent {
    q_table: QTable,
}

impl Agent {
    pub fn new(
        action_count: usize,
        learning_rate: f64,
        greedy_factor: f64,
        discount_factor: f64,
    ) -> Agent {
        Agent {
            q_table: QTable::get_instance(
                action_count,
                learning_rate,
                greedy_factor,
                discount_factor,
            ),
        }
    }

    pub fn train(
        &mut self,
        episode_count: usize,
        map: &Map,
        watch_training_progress: bool,
        watch_batch_size: usize,
        play_speed_millis: u64,
    ) {
        println!("Training started..");

        for episode in 1..episode_count + 1 {
            let mut state = map.get_player_init_state();
            let mut step_count = 0;

            while !map.is_win_state(state) && !map.is_loss_state(state) {
                step_count += 1;

                let next_action =
                    self.q_table
                        .get_next_best_action(state, map.get_dimension(), true)
                        as Action;

                let (future_state, rewards) =
                    apply_action_to_state(state, next_action, map, step_count);

                self.q_table.update_rewards(
                    state,
                    future_state,
                    next_action as usize,
                    rewards,
                    map.get_dimension(),
                );

                state = future_state;
            }

            if watch_training_progress && episode % watch_batch_size == 0 {
                io_utils::clear_screen();
                println!(
                    "Episode {} {}",
                    episode,
                    if map.is_win_state(state) {
                        SUCCEED_MSG
                    } else {
                        FAIL_MSG
                    }
                    .to_string()
                );

                print!(
                    "{}",
                    io_utils::render_q_table(
                        &self.q_table.get_snapshot(map.get_dimension()),
                        map.get_dimension()
                    )
                );

                thread::sleep(time::Duration::from_millis(play_speed_millis));
            }
        }
    }

    pub fn run_simulation(&mut self, map: &Map, max_step: u64, play_speed_millis: u64) {
        println!("Simulation run with the trained model:");
        let mut state = map.get_player_init_state();
        let mut step_count = 0;
        while !map.is_win_state(state) && !map.is_loss_state(state) {
            if step_count > max_step {
                println!("Reached max number of steps allowed {}", max_step);
                break;
            }

            step_count += 1;
            let next_action = self
                .q_table
                .get_next_best_action(state, map.get_dimension(), false);

            io_utils::clear_screen();
            print!("{}", io_utils::render_map(state, map));
            let (future_state, _) =
                apply_action_to_state(state, next_action as u8, map, step_count);
            state = future_state;

            thread::sleep(time::Duration::from_millis(play_speed_millis));
        }

        io_utils::clear_screen();
        print!("{}", io_utils::render_map(state, map));
        println!("Done running simulation.");
    }
}

fn apply_action_to_state(
    state: State,
    action: Action,
    map: &Map,
    step_count: u64,
) -> (State, Reward) {
    let (is_valid, future_state) = apply_action(state, action, map.get_dimension());

    let time_loss = step_count as f64 * LOSS_PER_TIME_UNIT;

    if !is_valid {
        return (state, time_loss);
    }

    if map.is_win_state(future_state) {
        return (future_state, REWARD + time_loss);
    }

    if map.is_loss_state(future_state) {
        return (future_state, PUNISHMENT + time_loss);
    }

    (future_state, time_loss)
}

fn apply_action(state: State, action: Action, dimension: Dimension) -> (bool, State) {
    match action {
        UP => {
            let updated_j = state.1 + 1;

            if updated_j >= dimension.1 {
                return (false, state);
            }

            return (true, (state.0, updated_j));
        }
        DOWN => {
            if state.1 < 1 {
                return (false, state);
            }

            return (true, (state.0, state.1 - 1));
        }
        LEFT => {
            if state.0 < 1 {
                return (false, state);
            }

            return (true, (state.0 - 1, state.1));
        }
        RIGHT => {
            let updated_i = state.0 + 1;

            if updated_i >= dimension.0 {
                return (false, state);
            }

            return (true, (updated_i, state.1));
        }
        _ => panic!("invalid action"),
    }
}
