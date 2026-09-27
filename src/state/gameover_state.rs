use std::collections::HashMap;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct GameOverState {
    pub button_manager: ButtonManager,
}

impl GameOverState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
        }
    }
}
