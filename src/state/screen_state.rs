use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ScreenType {
    Title,
    PlayerConfig,
    Credits,
    Settings,
    Game,
    GameOptions,
    GameOver,
    RoundOver,
}

#[turbo::serialize]
pub struct ScreenState {
    pub current_screen: ScreenType,
    pub next_screen: Option<ScreenType>,
    pub previous_screen: Option<ScreenType>,
    pub transition_started_ms: u64,
    pub transition_duration_ms: u64,
}

impl ScreenState {
    pub fn new() -> Self {
        Self {
            current_screen: ScreenType::Title,
            next_screen: None,
            previous_screen: None,
            transition_started_ms: 0,
            transition_duration_ms: 0,
        }
    }
}
