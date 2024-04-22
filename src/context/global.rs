use std::sync::Arc;

use tokio::sync::Mutex;

use crate::player::Player;

#[derive(Clone, Debug)]
pub struct GlobalContext {
    pub players: Arc<Mutex<Vec<Player>>>
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::new()))
        }
    }
}