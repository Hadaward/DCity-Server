use std::sync::Arc;

use tokio::sync::Mutex;

use crate::player::Player;
use super::state::State;

#[derive(Clone, Debug)]
pub struct GlobalContext {
    pub players: Arc<Mutex<Vec<Player>>>,
    pub player_count: Arc<Mutex<State<usize>>>
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::new())),
            player_count: Arc::new(Mutex::new(State::new(0))),
        }
    }
}