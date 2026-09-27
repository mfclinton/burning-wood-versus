use turbo::*;

use crate::{
    common::{bounds_get_bottom_center, Anchor, Renderable, ScaleMode, Vec2}, config, entities::{EntityRenderer, EntityState}, render_background, sprites, update_lights, App, GameState, PlayerState, Vec2Int
};

const EQUIPMENT_SLOT_JUST_PRESSED_COLOR: u32 = 0x666666FF;

// --- Rendering Functions ---

pub fn render_game(app: &mut App) {
    render_background(&app.game_state_manager.local_state.map_bounds);
    draw_game_elements(&app.game_state_manager.local_state, app.settings_state.debug_mode);

    debug_game_rendering(app);

    draw_equipment_slots(app);
    update_lights(&app.game_state_manager.local_state);
}

fn draw_game_elements(game_state: &GameState, debug_mode: bool) {
    for effect in game_state.active_effects.values() {
        effect.render(debug_mode);
    }
    
    // Top Player
    let winning_player_id = game_state.players.values()
        .max_by_key(|player| player.data().health)
        .map(|player| player.get_id());
    
    for player in game_state.players.values() {
        let is_winning = Some(player.get_id()) == winning_player_id;

        player.render(debug_mode);
        player.draw_player_username_with_crown(is_winning);
    }
    
    for projectile in game_state.projectiles.values() {
        projectile.render(debug_mode);
    }
}

fn draw_equipment_slots(app: &App) {
    if let Some(user_id) = turbo::os::client::user_id() {
        if let Some(client_player) = app.game_state_manager.local_state.players.get(&user_id) {
            draw_player_equipment_slots(client_player);
        }
    }
}

fn draw_player_equipment_slots(player: &PlayerState) {
    let canvas_bounds = bounds::screen();
    let player_data = player.data();
    let item_keys_just_pressed = PlayerState::get_item_keyboard_key_pressed();

    let scale = 0.1;
    let icon_spacing_frac = 1.05;

    let slot_origin: Vec2Int = canvas_bounds.translate_by_fraction(0.01, 0.015).top_left().into();
    for (i, just_pressed) in item_keys_just_pressed.iter().enumerate() {
        // Draw Slot Icon
        let slot_icon_sprite = sprites::UI_GAMESCREEN_ITEMSLOT.with_scale(scale, ScaleMode::HeightRelativeContain);
        let mut slot_bounds = slot_icon_sprite.get_bounds_anchored_at(slot_origin, Anchor::TopLeft);
        slot_bounds = slot_bounds.translate_x_by_fraction(icon_spacing_frac * i as f32);

        let tint_color = if *just_pressed { EQUIPMENT_SLOT_JUST_PRESSED_COLOR } else { 0xFFFFFFFF };
        sprite!(
            slot_icon_sprite.name,
            bounds = slot_bounds,
            color = tint_color,
            fixed = true
        );

        // Draw Item
        if let Some(item_state) = &player_data.held_items[i] {
            item_state.draw_item_in_equipment_slot(slot_bounds.clone());
        }

        // Draw Key
        if let Some(key_sprite) = config::ITEM_SLOT_SPRITES.get(i) {
            let key_scale = 0.025;

            let key_sprite = key_sprite.with_scale(key_scale, ScaleMode::HeightRelativeContain);
            let mut key_bounds = key_sprite.get_bounds_anchored_at(bounds_get_bottom_center(&slot_bounds), Anchor::TopCenter);
            key_bounds = key_bounds.translate_y_by_fraction(0.1);

            sprite!(
                key_sprite.name,
                bounds = key_bounds,
                color = tint_color,
                fixed = true
            );
        }
    }
}

// --- Debug ---

fn debug_game_rendering(app: &mut App) {
    if !app.settings_state.debug_mode {
        return;
    }

    app.game_state_manager.debug_client_player();
    app.game_state_manager.debug_projectiles();
}
