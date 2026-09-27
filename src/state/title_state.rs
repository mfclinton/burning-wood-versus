use std::collections::HashMap;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct TitleState {
    pub button_manager: ButtonManager,
}

impl TitleState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
        }
    }
}
