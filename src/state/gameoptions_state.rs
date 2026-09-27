use std::collections::HashMap;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct GameOptionsState {
    pub button_manager: ButtonManager,
}

impl GameOptionsState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
        }
    }
}
