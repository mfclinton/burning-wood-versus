use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_left_center, bounds_get_top_center, draw_button, ease, inset_bounds_by_fraction, inset_bounds_width_by_fraction, Anchor, Renderable, ScaleMode, Span, Text}, draw_fade_to_black, names, screen_management, sprites, transition_t, App, ScreenType, Vec2, JERSEY_20
};

const RANDOMIZE_BUTTON_INDEX: usize = 0;
const MULTIPLAYER_BUTTON_INDEX: usize = 1;

// Animation Timing Constants
const NAMEPLATE_SPAN: Span = Span::new(0.1, 0.5);
const RANDOMIZE_BUTTON_SPAN: Span = Span::new(0.3, 0.7);
const MULTIP_BUTTON_SPAN: Span = Span::new(0.5, 1.0);
const FADE_TO_BLACK_SPAN: Span = Span::new(0.5, 0.95);

// --- Screen Management Functions ---

pub fn enter(app: &mut App) {
    // Set Intro Transition
    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = 1000;

    // Load Username
    app.player_config_state.username = local::load()
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        .unwrap_or_default();
}

pub fn update(app: &mut App) {
    clear(0x000000ff);

    // Render Elements
    let t = transition_t(app);

    let canvas_bounds = bounds::screen();
    let nameplate_bounds = draw_nameplate_ui(app);
    draw_buttons(app, &nameplate_bounds, &canvas_bounds);

    draw_fade_to_black(t, FADE_TO_BLACK_SPAN);
}

pub fn exit(app: &mut App) {
    // Save Username
    local::save(app.player_config_state.username.as_bytes());
}

// --- Nameplate UI ---

fn draw_nameplate_ui(app: &mut App) -> Bounds {
    let canvas_bounds = bounds::screen();
    let scale = 0.5;
    
    // Nameplate Sprite
    let nameplate_sprite = sprites::UI_PLAYERCONFIG_NAMEPLATE.with_scale(scale, ScaleMode::WidthRelativeContain);
    let mut nameplate_bounds = nameplate_sprite.get_bounds_anchored_at(bounds_get_top_center(&canvas_bounds), Anchor::TopCenter);
    nameplate_bounds = nameplate_bounds.translate_y_by_fraction(0.1);

    sprite!(sprites::UI_PLAYERCONFIG_NAMEPLATE.name, bounds = nameplate_bounds, fixed = true, color = 0xFFFFFF_ff);

    // Draw Username Text
    let mut username_text = Text::new(&app.player_config_state.username, &JERSEY_20);
    username_text = username_text.with_scale(scale * 0.9, ScaleMode::WidthRelative);
    
    let text_bounds = username_text.get_bounds_anchored_at(nameplate_bounds.center().into(), Anchor::Center);    
    username_text.render(text_bounds.xy().into(), 0x000000_ff, true);
    
    nameplate_bounds
}

// --- Buttons ---

fn draw_buttons(app: &mut App, nameplate_bounds: &Bounds, canvas_bounds: &Bounds) {
    // Get Button Bounds
    let button_bounds = get_button_bounds(nameplate_bounds, canvas_bounds);

    // Update Button Manager
    let button_manager = &mut app.player_config_state.button_manager;
    button_manager.update(&button_bounds);
    
    let selected_index = button_manager.selected_index;
    let recently_pressed = time::now() - button_manager.time_selected_last_pressed < 200;
    let is_pressed = button_manager.selected_is_pressed(&button_bounds);
    
    // Draw Randomize Button
    draw_button(
        &button_bounds[RANDOMIZE_BUTTON_INDEX], 
        selected_index == RANDOMIZE_BUTTON_INDEX, 
        recently_pressed,
        &sprites::UI_PLAYERCONFIG_RANDOMIZE_BUTTON_NORMAL,
        &sprites::UI_PLAYERCONFIG_RANDOMIZE_BUTTON_SELECTED,
        0xFFFFFF_FF
    );

    // Draw Multiplayer Button
    draw_button(
        &button_bounds[MULTIPLAYER_BUTTON_INDEX], 
        selected_index == MULTIPLAYER_BUTTON_INDEX, 
        recently_pressed,
        &sprites::UI_PLAYERCONFIG_MULTIP_BUTTON_NORMAL,
        &sprites::UI_PLAYERCONFIG_MULTIP_BUTTON_SELECTED,
        0xFFFFFF_FF
    );

    // Button Actions
    if is_pressed {
        match selected_index {
            RANDOMIZE_BUTTON_INDEX => {
                app.player_config_state.username = names::random_username();
            }
            MULTIPLAYER_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Game, 1000);
            }
            _ => {}
        }
    }
}

fn get_button_bounds(nameplate_bounds: &Bounds, canvas_bounds: &Bounds) -> [Bounds; 2] {
    // Calculate Offset Positions
    let nameplate_left_center = bounds_get_left_center(nameplate_bounds);
    let canvas_bottom_center = bounds_get_bottom_center(canvas_bounds);

    // Randomize Button
    let randomize_button_sprite = sprites::UI_PLAYERCONFIG_RANDOMIZE_BUTTON_NORMAL.with_scale(0.1, ScaleMode::WidthRelativeContain);
    let randomize_button_bounds = randomize_button_sprite.get_bounds_anchored_at(nameplate_left_center.into(), Anchor::CenterRight)
        .translate_x_by_fraction(-0.2);

    // Multiplayer Button
    let multiplayer_button_sprite = sprites::UI_PLAYERCONFIG_MULTIP_BUTTON_NORMAL.with_scale(0.5, ScaleMode::HeightRelativeContain);
    let mut multiplayer_button_bounds = multiplayer_button_sprite.get_bounds_anchored_at(canvas_bottom_center.into(), Anchor::BottomCenter);
    multiplayer_button_bounds = multiplayer_button_bounds.translate_y_by_fraction(-0.1);

    // Set Bounds
    let mut button_bounds = [Bounds::default(); 2];
    button_bounds[RANDOMIZE_BUTTON_INDEX] = randomize_button_bounds;
    button_bounds[MULTIPLAYER_BUTTON_INDEX] = multiplayer_button_bounds;

    button_bounds
}