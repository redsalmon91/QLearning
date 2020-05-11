use std::fs;

use model::map::Map;

const WIN_STATE: &str = "W";
const LOSS_STATE: &str = "L";
const PLAYER_INIT_STATE: &str = "P";

pub fn read_map_from_file(file: String) -> Map {
    let map_file_content = fs::read_to_string(file).expect("Unable to read the given map file");
    let map_lines: Vec<&str> = map_file_content.split('\n').collect();

    let mut win_states = Vec::new();
    let mut loss_states = Vec::new();
    let mut player_state = (0u64, 0u64);

    let mut height = map_lines.iter().count();
    let mut width = 0;

    for (i, line) in map_lines.iter().enumerate() {
        if line.trim().is_empty() {
            height = height - 1;
            continue;
        }

        let map_states: Vec<&str> = line.split(',').collect();
        let line_width = map_states.iter().count();

        if i == 0 {
            width = line_width;
        } else {
            if width != line_width {
                panic!(
                    "Map file has mismatched line width {} {}",
                    width, line_width
                );
            }
        }

        for (j, state) in map_states.iter().enumerate() {
            match state {
                &WIN_STATE => win_states.push((i as u64, j as u64)),
                &LOSS_STATE => loss_states.push((i as u64, j as u64)),
                &PLAYER_INIT_STATE => player_state = (i as u64, j as u64),
                _ => {}
            }
        }
    }

    Map::new(
        player_state,
        win_states,
        loss_states,
        (height as u64, width as u64),
    )
}
