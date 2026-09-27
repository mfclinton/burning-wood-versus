use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, bounds_get_top_center, draw_button, ease, inset_bounds_by_fraction, inset_bounds_width_by_fraction, scale_bounds_from_center, Anchor, Renderable, ScaleMode, Span, Vec2Int}, draw_fade_to_black, draw_round_over_timer, screen_management, set_lighting_shader, sprites, transition_t, update_ambient_light, update_and_render_game_state, App, ScreenType
};

const CONTINUE_BUTTON_INDEX: usize = 0;

const PANEL_SPAN: Span = Span::new(0.1, 0.5);
const HEADER_SPAN: Span = Span::new(0.3, 0.7);
const BUTTONS_SPAN: Span = Span::new(0.5, 1.0);
const FADE_TO_BLACK_SPAN: Span = Span::new(0.5, 0.95);

pub fn enter(app: &mut App) {
    set_lighting_shader();
    update_ambient_light(0.0);

    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = 1000;
}

pub fn update(app: &mut App) {
    update_and_render_game_state(app);

    let t: f32 = transition_t(app);

    draw_roundover_menu(app, t);
    draw_round_over_timer(app.game_state_manager.server_state.round_over_end_time_ms);
    draw_fade_to_black(t, FADE_TO_BLACK_SPAN);

    update_ambient_light(t);
}

pub fn exit(app: &mut App) {
    shaders::reset();
}

fn draw_roundover_menu(app: &mut App, t: f32) {
    let canvas_bounds = bounds::screen();

    let panel_bounds = draw_roundover_panel(t, &canvas_bounds);
    draw_roundover_poster(t, &panel_bounds);
    // draw_roundover_header(t, &panel_bounds);
    // process_buttons(app, t, &panel_bounds);
}

fn draw_roundover_panel(t: f32, canvas_bounds: &Bounds) -> Bounds {
    let scale = 0.5;

    let panel_sprite = sprites::UI_GAMEOVER_PANEL.with_scale(scale, ScaleMode::Fit);
    let panel_bounds = panel_sprite.get_bounds_anchored_at(canvas_bounds.center().into(), Anchor::Center);

    let panel_alpha = PANEL_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let panel_color = 0xFFFFFF00 | (panel_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_PANEL.name, bounds = panel_bounds, fixed = true, color = panel_color);

    panel_bounds
}

fn draw_roundover_poster(t: f32, panel_bounds: &Bounds) {
    let scale = 1.0;

    let poster_sprite = sprites::UI_GAMEOVER_POSTER.with_scale(scale, ScaleMode::Fit);
    let poster_bounds = poster_sprite.get_bounds_anchored_at(panel_bounds.center().into(), Anchor::Center);

    let poster_alpha = PANEL_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let poster_color = 0xFFFFFF00 | (poster_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_POSTER.name, bounds = poster_bounds, fixed = true, color = poster_color);
}

fn draw_roundover_header(t: f32, panel_bounds: &Bounds) {
    let scale = 0.1;

    let header_sprite = sprites::UI_GAMEOVER_HEADER.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut header_bounds = header_sprite.get_bounds_anchored_at(bounds_get_top_center(&panel_bounds), Anchor::BottomCenter);
    header_bounds = header_bounds.translate_y_by_fraction(-0.08);
    header_bounds = inset_bounds_width_by_fraction(&header_bounds, 0.1);

    let header_alpha = HEADER_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let header_color = 0xFFFFFF00 | (header_alpha * 255.0) as u32;

    sprite!(sprites::UI_GAMEOVER_HEADER.name, bounds = header_bounds, fixed = true, color = header_color);
}

fn process_buttons(app: &mut App, t: f32, panel_bounds: &Bounds) {
    let button_bounds = get_button_bounds(t, panel_bounds);

    let button_manager = &mut app.roundover_state.button_manager;
    button_manager.update(&button_bounds);

    let selected_index = button_manager.selected_index;
    let selected_pressed = button_manager.selected_is_pressed(&button_bounds);
    let selected_recently_pressed = time::now() - button_manager.time_selected_last_pressed < 200;

    let button_alpha = BUTTONS_SPAN.value(t, 0.0, 1.0, ease::ease_out);
    let button_color = 0xFFFFFF00 | (button_alpha * 255.0) as u32;

    draw_button(&button_bounds[CONTINUE_BUTTON_INDEX], selected_index == CONTINUE_BUTTON_INDEX,
                selected_recently_pressed, &sprites::UI_BUTTON_RETRY_NORMAL,
                &sprites::UI_BUTTON_RETRY_SELECTED, button_color);

    if selected_pressed {
        match selected_index {
            CONTINUE_BUTTON_INDEX => {
                screen_management::trigger_transition_to_screen(app, ScreenType::Game, 1000);
            }
            _ => {}
        }
    }
}

fn get_button_bounds(t: f32, panel_bounds: &Bounds) -> [Bounds; 1] {
    let scale = 0.1;

    let continue_sprite = sprites::UI_BUTTON_RETRY_NORMAL.with_scale(scale, ScaleMode::HeightRelativeContain);
    let mut continue_bounds = continue_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&panel_bounds), Anchor::TopCenter);

    continue_bounds = continue_bounds.translate_y_by_fraction(0.1);

    let mut button_bounds = [Bounds::default(); 1];
    button_bounds[CONTINUE_BUTTON_INDEX] = continue_bounds;

    for bounds in &mut button_bounds {
        *bounds = inset_bounds_width_by_fraction(bounds, 0.05);
    }

    button_bounds
}