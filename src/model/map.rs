use model::def::{Dimension, State};

pub struct Map {
    player_init_state: State,
    win_states: Vec<State>,
    loss_states: Vec<State>,
    dimension: Dimension,
}

impl Map {
    pub fn new(
        player_init_state: State,
        win_states: Vec<State>,
        loss_states: Vec<State>,
        dimension: Dimension,
    ) -> Map {
        Map {
            player_init_state: player_init_state,
            win_states: win_states,
            loss_states: loss_states,
            dimension: dimension,
        }
    }

    pub fn get_player_init_state(&self) -> State {
        self.player_init_state
    }

    pub fn is_win_state(&self, state: State) -> bool {
        self.win_states.contains(&state)
    }

    pub fn is_loss_state(&self, state: State) -> bool {
        self.loss_states.contains(&state)
    }

    pub fn get_dimension(&self) -> Dimension {
        self.dimension
    }
}
