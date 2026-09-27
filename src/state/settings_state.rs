use turbo::*;

use crate::{ButtonManager};

#[turbo::serialize]
pub struct SettingsState {
    pub button_manager: ButtonManager,
    
    pub sfx_enabled: bool,
    pub music_enabled: bool,
    pub debug_mode: bool,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            button_manager: ButtonManager::new(),

            sfx_enabled: true,
            music_enabled: true,
            debug_mode: false,
        }
    }

    pub fn toggle_sfx(&mut self) {
        self.sfx_enabled = !self.sfx_enabled;
    }

    pub fn toggle_music(&mut self) {
        self.music_enabled = !self.music_enabled;
    }

    pub fn toggle_debug_mode(&mut self) {
        self.debug_mode = !self.debug_mode;
    }
}