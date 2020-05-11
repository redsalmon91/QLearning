extern crate rand;

mod agent;
mod model;
mod util;

use std::env;

use agent::q_learning::Agent;
use util::io_utils;
use util::map_utils;

const ARGV_COUNT: usize = 10;
const ACTION_COUNT: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != ARGV_COUNT {
        println!(
            "\n\nExpected {} arguments, found {}\n",
            ARGV_COUNT,
            args.len()
        );

        println!(
            "Usage:\n\tq_learning
                <map-file>
                <training-episode-count>
                <learning-rate>
                <greedy-factor>
                <discount-factor>
                <max-step>
                <watch-training-process>
                <watch-batch-size>
                <play-speed-millis>
                \n\n"
        );

        println!(
            "Example:\n\tq_learning examples/map.txt 10000 0.01 0.99 0.99 1000 true 1000 100 \n\n"
        );
        return;
    }

    let map_file = args[1].parse::<String>().unwrap();
    let episode_count = args[2].parse::<usize>().unwrap();
    let learning_rate = args[3].parse::<f64>().unwrap();
    let greedy_factor = args[4].parse::<f64>().unwrap();
    let discount_factor = args[5].parse::<f64>().unwrap();
    let max_step = args[6].parse::<u64>().unwrap();
    let watch_training_process = args[7].parse::<bool>().unwrap();
    let watch_batch_size = args[8].parse::<usize>().unwrap();
    let play_speed = args[9].parse::<u64>().unwrap();

    let mut agent = Agent::new(ACTION_COUNT, learning_rate, greedy_factor, discount_factor);
    let map = map_utils::read_map_from_file(map_file);

    print!(
        "Loaded map with size {} {}\n{}",
        map.get_dimension().0,
        map.get_dimension().1,
        io_utils::render_map(map.get_player_init_state(), &map)
    );

    println!("Map is loaded, press any key to start training");

    io_utils::pause();

    agent.train(
        episode_count,
        &map,
        watch_training_process,
        watch_batch_size,
        play_speed,
    );

    println!(
        "Agent has completed training with {} episodes, press any key to run simulation",
        episode_count
    );

    io_utils::pause();

    agent.run_simulation(&map, max_step, play_speed);
}
