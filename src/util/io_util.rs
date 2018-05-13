pub fn map_action_to_unicode(action: usize) -> char {
    match action {
        0 => '\u{2190}',
        1 => '\u{2192}',
        _ => panic!("Unknown action {}", action),
    }
}
