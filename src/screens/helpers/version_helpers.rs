use turbo::*;

use crate::{common::{Anchor, Renderable, ScaleMode, Sprite, Text}, JERSEY_20};

pub fn draw_version() {
    let canvas_bounds = bounds::screen();

    // Version String
    let version = env!("CARGO_PKG_VERSION");
    let version_str = format!("v{}", version);

    // Draw Version
    let mut version_text = Text::new(&version_str, &JERSEY_20);
    version_text = version_text.with_scale(0.025, ScaleMode::HeightRelativeContain);

    let mut version_text_bounds = version_text.get_bounds_anchored_at(canvas_bounds.bottom_left().into(), Anchor::BottomLeft);
    version_text_bounds = version_text_bounds.translate_x_by_fraction(0.1);

    version_text.render(version_text_bounds.xy().into(), 0xFFFFFFFF, true);
}
