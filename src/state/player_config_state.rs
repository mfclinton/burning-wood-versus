use crate::{ButtonManager, names};

#[turbo::serialize]
pub struct PlayerConfigState {
    pub button_manager: ButtonManager,
    pub username: String,
}

impl PlayerConfigState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
            username: names::random_username(),
        }
    }
}