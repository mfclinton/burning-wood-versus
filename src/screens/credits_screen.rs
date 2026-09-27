use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_top_center, draw_button, ease, Anchor, Renderable, ScaleMode, Span}, screen_management, screens::{draw_version, helpers::draw_title_background}, sprites, transition_t, App, ScreenType
};

const BACK_BUTTON_INDEX: usize = 0;

// Animation Timing Constants
const CREDITS_PANEL_SPAN: Span = Span::new(0.1, 0.4);
const BUTTON_SPAN: Span = Span::new(0.6, 1.0);

// --- Screen Management Functions ---

pub fn enter(app: &mut App) {
    // Set Intro Transition
    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = 1000;
}

pub fn update(app: &mut App) {
    clear(0x000000ff);

    // Render Elements
    let t = transition_t(app);

    draw_title_background();
    draw_credits_panel(t);
    draw_buttons(app, t);
    draw_version();
}

pub fn exit(app: &mut App) {

}


// --- Credits Panel ---

fn draw_credits_panel(t: f32) {
    let canvas_bounds = bounds::screen();
    let scale = 0.8;

    // Credits Panel
    let credits_sprite = sprites::UI_CREDITS_SCREEN_PANEL.with_scale(scale, ScaleMode::HeightRelativeContain);

    let mut start_credits_bounds = credits_sprite.get_bounds_anchored_at(bounds_get_top_center(&canvas_bounds), Anchor::BottomCenter);
    let end_credits_bounds = credits_sprite.get_bounds_anchored_at(canvas_bounds.center().into(), Anchor::Center);

    let credits_y_pos = CREDITS_PANEL_SPAN.value(t, start_credits_bounds.y() as f32, end_credits_bounds.y() as f32, ease::ease_out);
    
    let credits_bounds = credits_sprite.get_bounds().position_xy((start_credits_bounds.x(), credits_y_pos));
    sprite!(sprites::UI_CREDITS_SCREEN_PANEL.name, bounds = credits_bounds, fixed = true);
}

// --- Draw ---

fn draw_buttons(app: &mut App, t: f32) {
    let canvas_bounds = bounds::screen();

    // Buttons
    process_buttons(app, &canvas_bounds, t);
}

// --- Buttons ---

fn process_buttons(app: &mut App, parent_bounds: &Bounds, t: f32) {
    // Get Button Bounds
    let button_bounds = get_button_bounds(parent_bounds, t);

    // Get Selected Button
    let button_manager = &mut app.credits_state.button_manager;
    button_manager.update(&button_bounds);
    let selected_index = button_manager.selected_index;
    let selected_pressed = button_manager.selected_is_pressed(&button_bounds);
    let selected_recently_pressed = time::now() - button_manager.time_selected_last_pressed < 200;

    // Draw Buttons
    draw_button(
        &button_bounds[BACK_BUTTON_INDEX], 
        selected_index == BACK_BUTTON_INDEX, 
        selected_recently_pressed,
        &sprites::UI_BUTTON_BACK_NORMAL,
        &sprites::UI_BUTTON_BACK_SELECTED,
        0xFFFFFFFF
    );

    // Button Actions
    if selected_pressed {
        match selected_index {
            BACK_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Title, 1000);
            }
            _ => {}
        }
    }
}

fn get_button_bounds(parent_bounds: &Bounds, t: f32) -> [Bounds; 1] {
    let scale = 0.15;
    
    // Back Button
    let back_button_sprite = sprites::UI_BUTTON_BACK_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut back_button_bounds = back_button_sprite.get_bounds_anchored_at(bounds_get_bottom_center(parent_bounds), Anchor::TopCenter);

    let button_y_offset = BUTTON_SPAN.value(t, 0.0, -(back_button_bounds.h() as f32) * 1.05, ease::ease_out);
    back_button_bounds = back_button_bounds.translate_y(button_y_offset);

    // Set Bounds
    let mut button_bounds = [Bounds::default(); 1];
    button_bounds[BACK_BUTTON_INDEX] = back_button_bounds;

    button_bounds
}