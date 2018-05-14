pub fn map_action_to_unicode(action: usize) -> char {
    match action {
        0 => '\u{2190}',
        1 => '\u{2192}',
        _ => panic!("Unknown action {}", action),
    }
}

pub fn visualize_state_with_unicode(state: u64, state_count: u64) -> String {
    let mut state_representation = String::new();

    if state != 0 {
        state_representation.push('\u{1F4A3}');
    }

    for i in 1..state_count - 1  {
        if i == state {
            state_representation.push('\u{1F47B}');
            continue;
        }

        state_representation.push(' ');
    }

    if state != state_count -1 {
        state_representation.push('\u{1F3F0}');
    }

    state_representation
}
