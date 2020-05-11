use rand;
use rand::Rng;
use std::collections::HashMap;

use model::def::{Dimension, State};

pub struct QTable {
    action_count: usize,
    learning_rate: f64,
    greedy_factor: f64,
    discount_factor: f64,
    rewards_table: HashMap<u64, Vec<f64>>,
}

impl QTable {
    pub fn get_instance(
        action_count: usize,
        learning_rate: f64,
        greedy_factor: f64,
        discount_factor: f64,
    ) -> QTable {
        QTable {
            action_count: action_count,
            learning_rate: learning_rate,
            greedy_factor: greedy_factor,
            discount_factor: discount_factor,
            rewards_table: HashMap::new(),
        }
    }

    pub fn get_snapshot(&self, dimension: Dimension) -> Vec<Vec<f64>> {
        let mut q_value_snapshot = vec![vec![0f64; dimension.1 as usize]; dimension.0 as usize];

        for i in 0..dimension.0 {
            for j in 0..dimension.1 {
                let state_key = create_state_hash((i, j), dimension);
                let state_rewards = if self.rewards_table.contains_key(&state_key) {
                    get_max(self.rewards_table.get(&state_key).unwrap())
                } else {
                    0f64
                };

                q_value_snapshot[i as usize][j as usize] = state_rewards;
            }
        }

        q_value_snapshot
    }

    pub fn update_rewards(
        &mut self,
        state: State,
        after_state: State,
        action: usize,
        rewards: f64,
        dimension: Dimension,
    ) {
        let experience_future_max_rewards = get_max(
            self.rewards_table
                .entry(create_state_hash(after_state, dimension))
                .or_insert(vec![0f64; self.action_count]),
        );

        let action_to_rewards_table = self
            .rewards_table
            .entry(create_state_hash(state, dimension))
            .or_insert(vec![0f64; self.action_count]);

        action_to_rewards_table[action] = (1f64 - self.learning_rate)
            * action_to_rewards_table[action]
            + self.learning_rate * (rewards + self.discount_factor * experience_future_max_rewards);
    }

    pub fn get_next_best_action(
        &mut self,
        state: State,
        dimension: Dimension,
        use_random_move: bool,
    ) -> usize {
        let rewards_vector = self
            .rewards_table
            .entry(create_state_hash(state, dimension))
            .or_insert(vec![0f64; self.action_count]);
        let mut max = rewards_vector[0];
        let mut best_action = 0;
        for (action_index, rewards) in rewards_vector.iter().enumerate() {
            if *rewards > max {
                max = *rewards;
                best_action = action_index;
            }
        }

        if use_random_move && rand::random::<f64>() > self.greedy_factor {
            return rand::thread_rng().gen_range(0, self.action_count);
        }

        best_action
    }
}

fn create_state_hash(state: State, dimension: Dimension) -> u64 {
    state.0 * dimension.0.max(dimension.1) + state.1
}

fn get_max(vector: &Vec<f64>) -> f64 {
    let mut max = vector[0];
    for v in vector {
        if *v > max {
            max = *v;
        }
    }

    max
}
