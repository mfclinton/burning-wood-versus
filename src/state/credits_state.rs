use turbo::*;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct CreditsState {
    pub button_manager: ButtonManager,
}

impl CreditsState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),
        }
    }
}
