use std::collections::HashMap;
use rand;
use rand::Rng;

pub struct QTable {
    action_count: usize,
    learning_rate: f64,
    greedy_factor: f64,
    rewards_table: HashMap<u64, Vec<f64>>,
}

impl QTable {
    pub fn get_instance(action_count: usize, learning_rate: f64, greedy_factor: f64) -> QTable {
        QTable {
            action_count: action_count,
            learning_rate: learning_rate,
            greedy_factor: greedy_factor,
            rewards_table: HashMap::new(),
        }
    }

    pub fn update_rewards(&mut self, initial_state_hash_key: u64, after_state_hash_key: u64, action_id: usize, rewards: f64) {
        let future_state_max_rewards = get_max_rewards(self.rewards_table.entry(after_state_hash_key).or_insert(vec![0f64; self.action_count]));
        let action_to_rewards_table = self.rewards_table.entry(initial_state_hash_key).or_insert(vec![0f64; self.action_count]);
        action_to_rewards_table[action_id] = rewards + self.learning_rate * future_state_max_rewards;
    }

    pub fn get_next_best_action(&mut self, state_hash_key: u64, use_random_move: bool) -> usize {
        let rewards_vector = self.rewards_table.entry(state_hash_key).or_insert(vec![0f64; self.action_count]);
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

/*
clone().fold() makes use of extra memory/cpu, so I rather do a naive iteration to get the max value
*/
fn get_max_rewards(rewards_vector: &Vec<f64>) -> f64 {
    let mut max = rewards_vector[0];
    for rewards in rewards_vector {
        if *rewards > max {
            max = *rewards;
        }
    }

    max
}
