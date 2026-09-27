use turbo::*;

use super::*;
use crate::{App, ScreenType};

#[derive(PartialEq)]
pub enum TransitionPhase {
    None,
    Entering,
    Exiting,
}

// --- State Management Functions ---

pub fn update_current_screen(app: &mut App) {
    update_screen(app);
    process_screen_exit_transition(app);
}

pub fn trigger_transition_to_screen(app: &mut App, new_screen: ScreenType, transition_duration_ms: u64) {
    // Already Transitioning
    if app.screen_state.next_screen.is_some() {
        return;
    }
    
    // Start Transition
    app.screen_state.next_screen = Some(new_screen.clone());
    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = transition_duration_ms;

    // Instant Transition
    if transition_duration_ms <= 0 {
        instant_transition_to_screen(app, new_screen);
        return;
    }
}

// --- Screen Helper Functions ---

pub fn enter_screen(app: &mut App) {
    match app.screen_state.current_screen {
        ScreenType::Title => title_screen::enter(app),
        ScreenType::Settings => settings_screen::enter(app),
        ScreenType::Credits => credits_screen::enter(app),
        ScreenType::PlayerConfig => player_config_screen::enter(app),
        ScreenType::Game => game_screen::enter(app),
        ScreenType::GameOptions => gameoptions_screen::enter(app),
        ScreenType::GameOver => gameover_screen::enter(app),
        ScreenType::RoundOver => roundover_screen::enter(app),
    }
}

pub fn exit_screen(app: &mut App) {
    match app.screen_state.current_screen {
        ScreenType::Title => title_screen::exit(app),
        ScreenType::Settings => title_screen::enter(app),
        ScreenType::Credits => title_screen::enter(app),
        ScreenType::PlayerConfig => player_config_screen::exit(app),
        ScreenType::Game => game_screen::exit(app),
        ScreenType::GameOptions => gameoptions_screen::exit(app),
        ScreenType::GameOver => gameover_screen::exit(app),
        ScreenType::RoundOver => roundover_screen::exit(app),
    }
}

pub fn update_screen(app: &mut App) {
    match app.screen_state.current_screen {
        ScreenType::Title => title_screen::update(app),
        ScreenType::Settings => settings_screen::update(app),
        ScreenType::Credits => credits_screen::update(app),
        ScreenType::PlayerConfig => player_config_screen::update(app),
        ScreenType::Game => game_screen::update(app),
        ScreenType::GameOptions => gameoptions_screen::update(app),
        ScreenType::GameOver => gameover_screen::update(app),
        ScreenType::RoundOver => roundover_screen::update(app),
    }
}

// --- Transition Helpers ---

pub fn transition_t(app: &App) -> f32 {
    let now = time::now();
    let start = app.screen_state.transition_started_ms;
    let duration = app.screen_state.transition_duration_ms.max(1) as f32;
    
    let elapsed = (now - start) as f32;
    match transition_phase(app) {
        TransitionPhase::Entering => (elapsed / duration).clamp(0.0, 1.0),
        TransitionPhase::Exiting => (1.0 - (elapsed / duration)).clamp(0.0, 1.0),
        TransitionPhase::None => 1.0,
    }
}


fn transition_phase(app: &App) -> TransitionPhase {
    if app.screen_state.next_screen.is_some() {
        TransitionPhase::Exiting
    }
    else if app.screen_state.transition_duration_ms > 0 {
        TransitionPhase::Entering
    }
    else {
        TransitionPhase::None
    }
}

fn instant_transition_to_screen(app: &mut App, new_screen: ScreenType) {
    // Exit
    exit_screen(app);

    // Update State
    app.screen_state.previous_screen = Some(app.screen_state.current_screen.clone());
    app.screen_state.current_screen = new_screen;
    app.screen_state.next_screen = None;
    app.screen_state.transition_started_ms = 0;
    app.screen_state.transition_duration_ms = 0;

    // Enter
    enter_screen(app);
}

fn process_screen_exit_transition(app: &mut App) {
    // No Transition
    if transition_phase(app) != TransitionPhase::Exiting {
        return;
    }


    // Complete Transition
    let t = transition_t(app);
    if t <= 0.0 {
        let next_screen = app.screen_state.next_screen.take().unwrap();
        instant_transition_to_screen(app, next_screen);
    }
}
