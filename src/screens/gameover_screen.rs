use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_top_center, draw_button, ease, inset_bounds_by_fraction, inset_bounds_width_by_fraction, scale_bounds_from_center, Anchor, Renderable, ScaleMode, Span, Vec2Int}, draw_fade_to_black, screen_management, set_lighting_shader, sprites, transition_t, update_ambient_light, update_and_render_game_state, App, ScreenType
};

const RESPAWN_BUTTON_INDEX: usize = 0;
const MAIN_MENU_BUTTON_INDEX: usize = 1;

// Animation Timing Constants
const PANEL_SPAN: Span = Span::new(0.1, 0.5);
const HEADER_SPAN: Span = Span::new(0.3, 0.7);
const BUTTONS_SPAN: Span = Span::new(0.5, 1.0);
const FADE_TO_BLACK_SPAN: Span = Span::new(0.5, 0.95);

// --- Screen Management Functions ---

pub fn enter(app: &mut App) {
    // Initialize Shader
    set_lighting_shader();
    update_ambient_light(0.0);

    // Set Intro Transition
    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = 1000;
}

pub fn update(app: &mut App) {
    // Update and Render Core Game State
    update_and_render_game_state(app);

    // Screen-specific UI
    let t: f32 = transition_t(app);

    draw_gameover_menu(app, t);
    draw_fade_to_black(t, FADE_TO_BLACK_SPAN);

    update_ambient_light(t);
}

pub fn exit(app: &mut App) {
    // Disable Shader
    shaders::reset();
}

// --- Menu ---

fn draw_gameover_menu(app: &mut App, t: f32) {
    let canvas_bounds = bounds::screen();

    let panel_bounds = draw_gameover_panel(t, &canvas_bounds);
    draw_gameover_poster(t, &panel_bounds);
    draw_gameover_header(t, &panel_bounds);
    process_buttons(app, t, &panel_bounds);
}

fn draw_gameover_panel(t: f32, canvas_bounds: &Bounds) -> Bounds {
    let scale = 0.5;
    
    let panel_sprite = sprites::UI_GAMEOVER_PANEL.with_scale(scale, ScaleMode::Fit);
    let panel_bounds = panel_sprite.get_bounds_anchored_at(canvas_bounds.center().into(), Anchor::Center);
    
    let panel_alpha = PANEL_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let panel_color = 0xFFFFFF00 | (panel_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_PANEL.name, bounds = panel_bounds, fixed = true, color = panel_color);

    panel_bounds
}

fn draw_gameover_poster(t: f32, panel_bounds: &Bounds) {
    let scale = 1.0;
    
    let poster_sprite = sprites::UI_GAMEOVER_POSTER.with_scale(scale, ScaleMode::Fit);
    let poster_bounds = poster_sprite.get_bounds_anchored_at(panel_bounds.center().into(), Anchor::Center);

    let poster_alpha = PANEL_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let poster_color = 0xFFFFFF00 | (poster_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_POSTER.name, bounds = poster_bounds, fixed = true, color = poster_color);
}

fn draw_gameover_header(t: f32, panel_bounds: &Bounds) {
    let scale = 0.1;
    
    let header_sprite = sprites::UI_GAMEOVER_HEADER.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut header_bounds = header_sprite.get_bounds_anchored_at(bounds_get_top_center(&panel_bounds), Anchor::BottomCenter);
    header_bounds = header_bounds.translate_y_by_fraction(-0.08);
    header_bounds = inset_bounds_width_by_fraction(&header_bounds, 0.1);
    
    let header_alpha = HEADER_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let header_color = 0xFFFFFF00 | (header_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_HEADER.name, bounds = header_bounds, fixed = true, color = header_color);
}

// --- Buttons ---

fn process_buttons(app: &mut App, t: f32, panel_bounds: &Bounds) {
    // Get Button Bounds
    let button_bounds = get_button_bounds(t, panel_bounds);

    // Get Selected Button
    let button_manager = &mut app.gameover_state.button_manager;
    button_manager.update(&button_bounds);

    let selected_index = button_manager.selected_index;
    let selected_pressed = button_manager.selected_is_pressed(&button_bounds);
    let selected_recently_pressed = time::now() - button_manager.time_selected_last_pressed < 200;
    
    // Calculate Alpha
    let button_alpha = BUTTONS_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let button_color = 0xFFFFFF00 | (button_alpha * 255.0) as u32;

    // Draw Buttons
    draw_button(&button_bounds[RESPAWN_BUTTON_INDEX], selected_index == RESPAWN_BUTTON_INDEX,
                selected_recently_pressed, &sprites::UI_BUTTON_RETRY_NORMAL,
                &sprites::UI_BUTTON_RETRY_SELECTED, button_color);
    draw_button(&button_bounds[MAIN_MENU_BUTTON_INDEX], selected_index == MAIN_MENU_BUTTON_INDEX,
                selected_recently_pressed, &sprites::UI_BUTTON_BACK_NORMAL,
                &sprites::UI_BUTTON_BACK_SELECTED, button_color);

    if selected_pressed {
        match selected_index {
            RESPAWN_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Game, 1000);
            }
            MAIN_MENU_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Title, 1000);
            }
            _ => {}
        }
    }
}

fn get_button_bounds(t: f32, panel_bounds: &Bounds) -> [Bounds; 2] {
    let scale = 0.1;

    // Respawn Button
    let respawn_sprite = sprites::UI_BUTTON_RETRY_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut respawn_bounds = respawn_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&panel_bounds), Anchor::TopCenter);

    respawn_bounds = respawn_bounds.translate_y_by_fraction(0.1);

    // Main Menu Button
    let main_menu_sprite = sprites::UI_BUTTON_BACK_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut main_menu_bounds = main_menu_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&respawn_bounds), Anchor::TopCenter);

    main_menu_bounds = main_menu_bounds.translate_y_by_fraction(0.1);

    // Set Bounds
    let mut button_bounds = [Bounds::default(); 2];
    button_bounds[RESPAWN_BUTTON_INDEX] = respawn_bounds;
    button_bounds[MAIN_MENU_BUTTON_INDEX] = main_menu_bounds;

    // Inset Bounds
    for bounds in &mut button_bounds {
        *bounds = inset_bounds_width_by_fraction(bounds, 0.05);
    }

    button_bounds
}
