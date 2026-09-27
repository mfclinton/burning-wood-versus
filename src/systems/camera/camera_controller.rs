use turbo::*;

use crate::{entities::EntityState, GameState, PlayerState, Vec2};

pub const CAMERA_FOLLOW_SPEED: f32 = 0.05;
pub const CAMERA_DEADZONE_RADIUS: f32 = 30.0;
pub const CAMERA_VELOCITY_DAMPING: f32 = 0.9;
pub const CAMERA_VELOCITY_SMOOTHING: f32 = 0.1;

pub const CAMERA_PLAYER_SCREEN_PERCENTAGE: f32 = 0.005;
pub const CAMERA_ZOOM_SMOOTHING: f32 = 0.05;

#[turbo::serialize]
pub struct CameraController {
    position: Vec2,
    velocity: Vec2,
    current_zoom: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            current_zoom: 1.0,
        }
    }

    pub fn update_with_game(&mut self, game_state: &GameState) {
        if let Some(user_id) = &turbo::os::client::user_id() {
            if let Some(player) = game_state.players.get(user_id) {
                self.update_with_player(player);
            }
        }
    }

    pub fn update_with_player(&mut self, player: &PlayerState) {
        self.update_zoom(player);
        self.update_position(player);
        self.apply_to_camera();
    }

    fn update_zoom(&mut self, player: &PlayerState) {
        let canvas_bounds = bounds::screen();
        let player_data = player.data();

        // Screen Space
        let total_screen_area = canvas_bounds.w() * canvas_bounds.h();
        let desired_player_screen_area = total_screen_area as f32 * CAMERA_PLAYER_SCREEN_PERCENTAGE;

        // World Space
        let player_world_area = std::f32::consts::PI * player_data.collider.radius * player_data.collider.radius;

        // Zoom
        let target_zoom = (desired_player_screen_area / player_world_area).sqrt();
        
        self.current_zoom = self.current_zoom + (target_zoom - self.current_zoom) * CAMERA_ZOOM_SMOOTHING;
    }

    fn update_position(&mut self, player: &PlayerState) {
        let player_data = player.data();
        
        let player_pos = player_data.collider.position;
        let scaled_deadzone = CAMERA_DEADZONE_RADIUS / self.current_zoom;
        let distance = (player_pos - self.position).length();
        
        // Inside Deadzone
        if distance <= scaled_deadzone {
            self.velocity *= CAMERA_VELOCITY_DAMPING;
        }
        // Outside Deadzone
        else {
            let direction = (player_pos - self.position).normalized();
            let target_velocity = direction * (distance - scaled_deadzone) * CAMERA_FOLLOW_SPEED;
            self.velocity = self.velocity.lerp(&target_velocity, 1.0 - CAMERA_VELOCITY_SMOOTHING);
        }
        
        self.position += self.velocity;
    }

    fn apply_to_camera(&self) {
        camera::set_xy(self.position.x, self.position.y);
        camera::set_z(self.current_zoom);
    }
}
