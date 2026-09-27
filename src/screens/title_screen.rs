use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_top_center, draw_button, ease, inset_bounds_width_by_fraction, Anchor, Renderable, ScaleMode, Span, Vec2, Vec2Int, play_ui_sound_if_enabled, ui_sounds}, draw_version, draw_fade_to_black, screen_management, screens::helpers::draw_title_background, sprites, transition_t, App, ScreenType
};

const PLAY_BUTTON_INDEX: usize = 0;
const SETTINGS_BUTTON_INDEX: usize = 1;
const CREDITS_BUTTON_INDEX: usize = 2;

// Animation Timing Constants
const TITLE_SPAN: Span = Span::new(0.2, 0.4);
const VS_SPAN: Span = Span::new(0.4, 0.7);
const POST_SPAN: Span = Span::new(0.6, 1.0);
const FADE_TO_BLACK_SPAN: Span = Span::new(0.1, 0.95);

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
    draw_logo(t);
    draw_post_and_buttons(app, t);
    draw_version();

    // Fade to Black
    let fade_to_black_exempt = [Some(ScreenType::Settings), Some(ScreenType::Credits)];
    if !fade_to_black_exempt.contains(&app.screen_state.next_screen) && !fade_to_black_exempt.contains(&app.screen_state.previous_screen) {
        draw_fade_to_black(t, FADE_TO_BLACK_SPAN);
    }
}

pub fn exit(app: &mut App) {

}

// --- Logo ---

fn draw_logo(t: f32) {
    let canvas_bounds = bounds::screen();
    let scale = 0.2;

    // Title
    let title_sprite = sprites::UI_LOGO_BURNINGWOOD.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut title_bounds = title_sprite.get_bounds_anchored_at(bounds_get_top_center(&canvas_bounds), Anchor::BottomCenter);

    let title_y_offset = TITLE_SPAN.value(t, 0.0, title_bounds.h() as f32 * 1.05, ease::ease_out);
    title_bounds = title_bounds.translate_y(title_y_offset);
    title_bounds = inset_bounds_width_by_fraction(&title_bounds, 0.05);

    sprite!(sprites::UI_LOGO_BURNINGWOOD.name, bounds = title_bounds, fixed = true);

    // VS
    let vs_sprite = sprites::UI_LOGO_VS.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut vs_bounds = vs_sprite.get_bounds_anchored_at(bounds_get_top_center(&canvas_bounds), Anchor::BottomCenter);

    let vs_y_offset = VS_SPAN.value(t, 0.0, (title_bounds.h() + vs_bounds.h()) as f32 * 1.05, ease::ease_out);
    vs_bounds = vs_bounds.translate_y(vs_y_offset);

    sprite!(sprites::UI_LOGO_VS.name, bounds = vs_bounds, fixed = true);
}

// --- Post and Buttons ---

fn draw_post_and_buttons(app: &mut App, t: f32) {
    let canvas_bounds = bounds::screen();
    let scale = 0.5;

    // Post
    let post_sprite = sprites::UI_TITLESCREEN_POST.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut post_bounds = post_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&canvas_bounds), Anchor::TopCenter);

    let post_y_offset = POST_SPAN.value(t, 0.0, -(post_bounds.h() as f32), ease::ease_out);
    post_bounds = post_bounds.translate_y(post_y_offset);

    sprite!(sprites::UI_TITLESCREEN_POST.name, bounds = post_bounds, fixed = true);

    // Buttons
    process_buttons(app, &post_bounds);
}

// --- Buttons ---

fn process_buttons(app: &mut App, parent_bounds: &Bounds) {
    // Get Button Bounds
    let button_bounds = get_button_bounds(parent_bounds);

    // Get Selected Button
    let button_manager = &mut app.title_state.button_manager;
    button_manager.update(&button_bounds);

    let selection_changed = button_manager.selection_changed();
    let selected_index = button_manager.selected_index;
    let selected_pressed = button_manager.selected_is_pressed(&button_bounds);
    let selected_recently_pressed = time::now() - button_manager.time_selected_last_pressed < 200;

    if selection_changed {
        play_ui_sound_if_enabled(app, ui_sounds::BUTTON_HOVER);
    }

    // Draw Buttons
    draw_button(
        &button_bounds[PLAY_BUTTON_INDEX], 
        selected_index == PLAY_BUTTON_INDEX, 
        selected_recently_pressed,
        &sprites::UI_TITLESCREEN_ENTERBUTTON_NORMAL,
        &sprites::UI_TITLESCREEN_ENTERBUTTON_SELECTED,
        0xFFFFFFFF
    );
    
    draw_button(
        &button_bounds[SETTINGS_BUTTON_INDEX], 
        selected_index == SETTINGS_BUTTON_INDEX, 
        selected_recently_pressed,
        &sprites::UI_TITLESCREEN_SETTINGSBUTTON_NORMAL,
        &sprites::UI_TITLESCREEN_SETTINGSBUTTON_SELECTED,
        0xFFFFFFFF
    );
    
    draw_button(
        &button_bounds[CREDITS_BUTTON_INDEX], 
        selected_index == CREDITS_BUTTON_INDEX, 
        selected_recently_pressed,
        &sprites::UI_TITLESCREEN_CREDITSBUTTON_NORMAL,
        &sprites::UI_TITLESCREEN_CREDITSBUTTON_SELECTED,
        0xFFFFFFFF
    );

    // Button Actions
    if selected_pressed {
        match selected_index {
            PLAY_BUTTON_INDEX => {
                play_ui_sound_if_enabled(app, ui_sounds::BUTTON_SELECT);
                screen_management::trigger_transition_to_screen(app, ScreenType::PlayerConfig, 1000);
            },
            SETTINGS_BUTTON_INDEX => {
                play_ui_sound_if_enabled(app, ui_sounds::BUTTON_SELECT);
                screen_management::trigger_transition_to_screen(app, ScreenType::Settings, 1000);
            },
            CREDITS_BUTTON_INDEX => {
                play_ui_sound_if_enabled(app, ui_sounds::BUTTON_SELECT);
                screen_management::trigger_transition_to_screen(app, ScreenType::Credits, 1000);
            },
            _ => {}
        }
    }
}

fn get_button_bounds(parent_bounds: &Bounds) -> [Bounds; 3] {
    let scale = 0.15;

    // Calculate Offset Positions
    let parent_top_center: Vec2Int = Vec2Int::into(bounds_get_top_center(parent_bounds)); // TODO

    // Play Button
    let play_button_sprite = sprites::UI_TITLESCREEN_ENTERBUTTON_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let play_button_bounds = play_button_sprite.get_bounds_anchored_at(parent_top_center.into(), Anchor::TopCenter);

    // Settings Button
    let settings_button_sprite = sprites::UI_TITLESCREEN_SETTINGSBUTTON_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let settings_button_bounds = settings_button_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&play_button_bounds), Anchor::TopCenter);

    // Credits Button
    let credits_button_additional_offset = parent_bounds.h() as f32 * 0.02;
    let credits_button_sprite = sprites::UI_TITLESCREEN_CREDITSBUTTON_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let credits_button_bounds = credits_button_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&settings_button_bounds), Anchor::TopCenter).translate_y(credits_button_additional_offset);

    // Set Bounds
    let mut button_bounds = [Bounds::default(); 3];
    button_bounds[PLAY_BUTTON_INDEX] = play_button_bounds;
    button_bounds[SETTINGS_BUTTON_INDEX] = settings_button_bounds;
    button_bounds[CREDITS_BUTTON_INDEX] = credits_button_bounds;

    // Inset Bounds
    for bounds in &mut button_bounds {
        *bounds = inset_bounds_width_by_fraction(bounds, 0.05);
    }

    button_bounds
}
