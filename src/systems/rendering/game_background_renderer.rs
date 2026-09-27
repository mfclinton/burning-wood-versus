use turbo::*;

use crate::{common::Renderable, sprites};

pub fn render_background(map_bounds: &Bounds) {
    let camera_world_bounds = bounds::world();
    let background_sprite = &sprites::WORLD_BRICK;
    let outside_background_sprite = &sprites::WORLD_WALL;

    let background_sprite_size = background_sprite.scaled_size_int();

    // Grid Bounds
    let start_tile_x = (camera_world_bounds.left() as f32 / background_sprite_size.x as f32).floor() as i32;
    let start_tile_y = (camera_world_bounds.top() as f32 / background_sprite_size.y as f32).floor() as i32;
    let end_tile_x = (camera_world_bounds.right() as f32 / background_sprite_size.x as f32).ceil() as i32;
    let end_tile_y = (camera_world_bounds.bottom() as f32 / background_sprite_size.y as f32).ceil() as i32;

    // Render Tiles
    for tile_y in start_tile_y..=end_tile_y {
        for tile_x in start_tile_x..=end_tile_x {
            let world_x = tile_x * background_sprite_size.x as i32;
            let world_y = tile_y * background_sprite_size.y as i32;

            // In Play Area Check
            let tile_bounds = Bounds::new(world_x, world_y, background_sprite_size.x as u32, background_sprite_size.y as u32);
            let sprite = if map_bounds.contains(&tile_bounds) {
                background_sprite
            } else {
                outside_background_sprite
            };

            sprite!(
                sprite.name,
                x = world_x,
                y = world_y,
                w = background_sprite_size.x,
                h = background_sprite_size.y
            );
        }
    }
}
