use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_top_center, draw_button, ease, Anchor, Renderable, ScaleMode, Span, Vec2}, screen_management, screens::{draw_version, helpers::draw_title_background}, sprites, transition_t, App, ScreenType
};

const TOGGLE_1_BUTTON_INDEX: usize = 0;
const TOGGLE_2_BUTTON_INDEX: usize = 1;
const TOGGLE_3_BUTTON_INDEX: usize = 2;
const BACK_BUTTON_INDEX: usize = 3;

// Animation Timing Constants
const SETTINGS_PANEL_SPAN: Span = Span::new(0.1, 0.4);
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
    draw_settings_panel(app, t);
    draw_version();
}

pub fn exit(app: &mut App) {

}

// --- Settings Panel ---

fn draw_settings_panel(app: &mut App, t: f32) {
    let canvas_bounds = bounds::screen();
    let scale = 0.8;

    // Settings Panel
    let settings_sprite = sprites::UI_SETTINGS_SCREEN_PANEL.with_scale(scale, ScaleMode::HeightRelativeContain);

    let mut start_settings_bounds = settings_sprite.get_bounds_anchored_at(bounds_get_top_center(&canvas_bounds), Anchor::BottomCenter);
    let end_settings_bounds = settings_sprite.get_bounds_anchored_at(canvas_bounds.center().into(), Anchor::Center);

    let settings_y_pos = SETTINGS_PANEL_SPAN.value(t, start_settings_bounds.y() as f32, end_settings_bounds.y() as f32, ease::ease_out);
    
    let settings_bounds = settings_sprite.get_bounds().position_xy((start_settings_bounds.x(), settings_y_pos));
    sprite!(sprites::UI_SETTINGS_SCREEN_PANEL.name, bounds = settings_bounds, fixed = true);

    // Buttons
    let button_bounds = get_button_bounds(&settings_bounds, t);
    process_buttons(app, button_bounds);
}

// --- Buttons ---

fn process_buttons(app: &mut App, button_bounds: [Bounds; 4]) {
    // Get Selected Button
    let button_manager = &mut app.settings_state.button_manager;
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

    // Draw Toggle Buttons
    draw_toggle_button(&button_bounds[TOGGLE_1_BUTTON_INDEX], selected_index == TOGGLE_1_BUTTON_INDEX, selected_recently_pressed, app.settings_state.sfx_enabled);
    draw_toggle_button(&button_bounds[TOGGLE_2_BUTTON_INDEX], selected_index == TOGGLE_2_BUTTON_INDEX, selected_recently_pressed, app.settings_state.music_enabled);
    draw_toggle_button(&button_bounds[TOGGLE_3_BUTTON_INDEX], selected_index == TOGGLE_3_BUTTON_INDEX, selected_recently_pressed, app.settings_state.debug_mode);

    // Button Actions
    if selected_pressed {
        match selected_index {
            BACK_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Title, 1000);
            }
            TOGGLE_1_BUTTON_INDEX => {
                app.settings_state.toggle_sfx();
            }
            TOGGLE_2_BUTTON_INDEX => {
                app.settings_state.toggle_music();
            }
            TOGGLE_3_BUTTON_INDEX => {
                app.settings_state.toggle_debug_mode();
            }
            _ => {}
        }
    }
}

fn get_button_bounds(settings_bounds: &Bounds, t: f32) -> [Bounds; 4] {
    let canvas_bounds = bounds::screen();
    
    // Back Button
    let back_button_scale = 0.12;

    let back_button_sprite = sprites::UI_BUTTON_BACK_NORMAL.with_scale(back_button_scale, ScaleMode::HeightRelativeContain);
    let mut back_button_bounds = back_button_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&canvas_bounds), Anchor::TopCenter);

    let button_y_offset = BUTTON_SPAN.value(t, 0.0, -(back_button_bounds.h() as f32) * 1.05, ease::ease_out);
    back_button_bounds = back_button_bounds.translate_y(button_y_offset);

    // Toggle Buttons
    let settings_origin = Vec2::from(settings_bounds.xy());
    let settings_dims = Vec2::from(settings_bounds.wh());

    let toggle_button_scale = settings_dims.x / sprites::UI_SETTINGS_SCREEN_PANEL.native_size().x;
    let toggle_sprite = sprites::UI_TOGGLE_INACTIVE.with_scale(toggle_button_scale, ScaleMode::Absolute);
    
    let toggle_1_pos = settings_origin + settings_dims * Vec2::new(0.5, 0.25);
    let toggle_1_bounds = toggle_sprite.get_bounds_anchored_at(toggle_1_pos.into(), Anchor::Center);
    
    let toggle_2_pos = settings_origin + settings_dims * Vec2::new(0.5, 0.5);
    let toggle_2_bounds = toggle_sprite.get_bounds_anchored_at(toggle_2_pos.into(), Anchor::Center);
    
    let toggle_3_pos = settings_origin + settings_dims * Vec2::new(0.5, 0.75);
    let toggle_3_bounds = toggle_sprite.get_bounds_anchored_at(toggle_3_pos.into(), Anchor::Center);

    // Set Bounds
    let mut button_bounds = [Bounds::default(); 4];
    button_bounds[BACK_BUTTON_INDEX] = back_button_bounds;
    button_bounds[TOGGLE_1_BUTTON_INDEX] = toggle_1_bounds;
    button_bounds[TOGGLE_2_BUTTON_INDEX] = toggle_2_bounds;
    button_bounds[TOGGLE_3_BUTTON_INDEX] = toggle_3_bounds;

    button_bounds
}

fn draw_toggle_button(bounds: &Bounds, is_selected: bool, recently_pressed: bool, is_active: bool) {
    let (normal_sprite, selected_sprite) = if is_active {
        (&sprites::UI_TOGGLE_ACTIVE, &sprites::UI_TOGGLE_ACTIVE_SELECTED)
    } else {
        (&sprites::UI_TOGGLE_INACTIVE, &sprites::UI_TOGGLE_INACTIVE_SELECTED)
    };

    draw_button(
        bounds,
        is_selected,
        recently_pressed,
        normal_sprite,
        selected_sprite,
        0xFFFFFFFF
    );
}
