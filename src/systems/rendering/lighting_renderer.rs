
use turbo::*;

use crate::{
    common::{lerp, Anchor, Vec2Int},
    config, shaders, App, GameState,
    EntityState
};

const AMBIENT_LIGHT: [f32; 3] = [0.1, 0.1, 0.1];
const FULL_AMBIENT_LIGHT: [f32; 3] = [1.0, 1.0, 1.0];

const MAX_LIGHTS: usize = 4;

const LIGHT_COLOR: [f32; 3] = [1.0, 0.8, 0.6];
const LIGHT_INTENSITY_MULTIPLIER: f32 = 8.0;
const LIGHT_RADIUS_MULTIPLIER: f32 = 0.2;

// --- Lighting ---

pub fn set_lighting_shader() {
    #[cfg(feature = "lighting")]
    {
        shaders::set("light-shader");
    }
}

pub fn update_ambient_light(t: f32) {
    #[cfg(feature = "lighting")]
    {
        // Lerp
        let new_ambient_light = [
            lerp(AMBIENT_LIGHT[0], FULL_AMBIENT_LIGHT[0], t),
            lerp(AMBIENT_LIGHT[1], FULL_AMBIENT_LIGHT[1], t),
            lerp(AMBIENT_LIGHT[2], FULL_AMBIENT_LIGHT[2], t),
        ];
        turbo::canvas::lights::set_ambient_light(new_ambient_light);
    }
}

pub fn update_lights(game_state: &GameState) {
    #[cfg(feature = "lighting")]
    {
        update_player_lights(game_state);
    }
}

#[cfg(feature = "lighting")]
fn update_player_lights(game_state: &GameState) {
    use crate::common::Vec2;

    let (camera_x, camera_y) = camera::xy();
    let camera_pos = Vec2::new(camera_x, camera_y);

    // Sort Distances
    let mut players_with_distance: Vec<_> = game_state.players.values()
        .map(|player| {
            let player_data = player.data();
            let distance = player_data.collider.position.distance_to(&camera_pos);
            (player, distance)
        })
        .collect();
    
    players_with_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Closest Lights
    let num_lights = players_with_distance.len().min(MAX_LIGHTS);
    turbo::canvas::lights::set_num_lights(num_lights as u32);
    
    for (i, (player, _)) in players_with_distance.iter().enumerate().take(num_lights) {
        let player_data = player.data();

        let player_radius = player_data.collider.radius;
        let player_position = player_data.collider.position;

        let light_radius = player_radius * LIGHT_RADIUS_MULTIPLIER;
        let light_intensity = player_radius * LIGHT_INTENSITY_MULTIPLIER;

        turbo::canvas::lights::set_light(
            i as u32,
            [player_position.x, player_position.y, 1.0],
            light_radius,
            LIGHT_COLOR,
            light_intensity
        );
    }
}
