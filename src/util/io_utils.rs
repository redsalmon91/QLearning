use std::io::{stdin, Read};

use model::def::{Dimension, State};
use model::map::Map;

const LION: char = '🦁';
const CASTLE: char = '🏰';
const ELF: char = '🧝';
const EMPTY: char = '🌳';
const POTENTIAL: char = '🏁';
const STAR: char = '🌟';
const WHITE_FLAG: char = '🏳';
const NEW_LINE: char = '\n';

const REWARD_THRESHOLD: f64 = 0f64;

pub fn render_map(state: State, map: &Map) -> String {
    let mut map_str = String::new();
    let dimension = map.get_dimension();

    for i in 0..dimension.0 {
        let mut line = String::new();

        for j in 0..dimension.1 {
            let index = (i, j);

            if map.is_win_state(index) {
                if index == state {
                    line.push(STAR);
                } else {
                    line.push(CASTLE);
                }
            } else if map.is_loss_state(index) {
                if index == state {
                    line.push(WHITE_FLAG);
                } else {
                    line.push(LION);
                }
            } else if index == state {
                line.push(ELF);
            } else {
                line.push(EMPTY);
            }
        }

        map_str.push_str(&line);
        map_str.push(NEW_LINE);
    }

    map_str
}

pub fn render_q_table(q_value_snapshot: &Vec<Vec<f64>>, dimension: Dimension) -> String {
    let mut q_value_table = String::new();

    for i in 0..dimension.0 {
        let mut line = String::new();

        for j in 0..dimension.1 {
            let reward = q_value_snapshot[i as usize][j as usize];

            if reward > REWARD_THRESHOLD {
                line.push(POTENTIAL);
            } else {
                line.push(EMPTY);
            }
        }

        q_value_table.push_str(&line);
        q_value_table.push(NEW_LINE);
    }

    q_value_table
}

pub fn clear_screen() {
    print!("\x1B[2J");
}

pub fn pause() {
    stdin().read(&mut [0u8]).unwrap();
}
