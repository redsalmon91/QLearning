use std::f64;

pub type Action = u8;
pub type State = (u64, u64);
pub type Dimension = (u64, u64);
pub type Reward = f64;

pub const REWARD: f64 = f64::MAX;
pub const PUNISHMENT: f64 = f64::MIN;
pub const LOSS_PER_TIME_UNIT: f64 = -1f64;
