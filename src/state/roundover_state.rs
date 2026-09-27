use std::collections::HashMap;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct RoundOverState {
    pub button_manager: ButtonManager,
}

impl RoundOverState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
        }
    }
}