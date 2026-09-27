use turbo::*;
use std::collections::HashSet;

mod config;

mod common;
use common::*;

mod state;
use state::*;

mod screens;
use screens::*;

mod systems;
use systems::*;

mod entities;
use entities::*;

mod networking;
use networking::*;

mod sprites;
use sprites::*;

mod fonts;
use fonts::*;

mod names;
use names::*;

#[turbo::game]
struct App {
    cur_frame_start_time: u64,
    last_frame_start_time: u64,
    circular_pad: CircularPad,
    camera_controller: CameraController,
    screen_state: ScreenState,
    title_state: TitleState,
    settings_state: SettingsState,
    credits_state: CreditsState,
    player_config_state: PlayerConfigState,
    game_state_manager: GameStateManager,
    gameoptions_state: GameOptionsState,
    gameover_state: GameOverState,
    roundover_state: RoundOverState,
    input_state: InputState,
    input_batch_timer: ClientTimer,
    join_request_timer: ClientTimer,
    pending_entity_lookups: HashSet<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            cur_frame_start_time: 0,
            last_frame_start_time: 0,
            circular_pad: CircularPad::default(),
            camera_controller: CameraController::new(),
            screen_state: ScreenState::new(),
            title_state: TitleState::new(),
            settings_state: SettingsState::new(),
            credits_state: CreditsState::new(),
            player_config_state: PlayerConfigState::new(),
            game_state_manager: GameStateManager::new(),
            gameoptions_state: GameOptionsState::new(),
            gameover_state: GameOverState::new(),
            roundover_state: RoundOverState::new(),
            input_state: InputState::new(),
            input_batch_timer: ClientTimer::new(config::CLIENT_INPUT_BATCH_RATE_MS),
            join_request_timer: ClientTimer::new(config::PLAYER_JOIN_REQUEST_INTERVAL_MS),
            pending_entity_lookups: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        // Debug
        let keyboard = keyboard::get();
        if keyboard.key_l().just_pressed() {
            *self = App::new();
        }
        else if keyboard.key_m().just_pressed() {
            self.game_state_manager = GameStateManager::new();
        }


        self.cur_frame_start_time = time::now();

        // Variables
        let time_scale = self.time_scale_relative_server();
        self.game_state_manager.set_time_scale(time_scale);

        // Maintain Connection
        let screen_state_connect_whitelist = vec![ScreenType::Game, ScreenType::GameOver];
        if screen_state_connect_whitelist.contains(&self.screen_state.current_screen) {
            connect();
        }

        // Initial Enter Screen
        if time::tick() == 0 {
            enter_screen(self);
        }

        // Update Screen
        screen_management::update_current_screen(self);
        // App::draw_help_message();

        // Update Audio
        update_background_music(self);

        // Update State
        self.last_frame_start_time = self.cur_frame_start_time;
    }

    // Time
    pub fn time_delta(&self) -> f32 {
       self.time_delta_ms() / 1000.0
    }

    pub fn time_delta_ms(&self) -> f32 {
        (self.cur_frame_start_time - self.last_frame_start_time) as f32
    }

    pub fn time_scale_relative_server(&self) -> f32 {
        let frame_time_ms = self.time_delta_ms();
        frame_time_ms / (config::SERVER_SIMULATION_RATE_MS as f32)
    }

    // Playtesting
    pub fn draw_help_message() {
        let canvas_bounds = bounds::screen();

        let help_text = Text::new("press 'M' if you're stuck", &JERSEY_20);
        let help_text_with_scale = help_text.with_scale(0.020, ScaleMode::HeightRelative);

        let mut help_text_bounds = help_text_with_scale.get_bounds_anchored_at(canvas_bounds.bottom_right().into(), Anchor::BottomRight);
        help_text_bounds = help_text_bounds.translate_by_fraction(-0.01, 0.03);

        let text_color = 0xFFFFFFFF; // White text
        help_text_with_scale.render(help_text_bounds.xy().into(), text_color, true);
    }
}
