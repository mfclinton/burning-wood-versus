use turbo::*;

use crate::{common::{Anchor, ClientTimer, Renderable, ScaleMode, Text}, config, JERSEY_20};

pub fn draw_round_timer(round_end_time_ms: Option<u64>) {
    let canvas_bounds = bounds::screen();

    let timer_str = if let Some(end_time) = round_end_time_ms {
        let current_time = time::now();
        if current_time >= end_time {
            "Round Over".to_string()
        } else {
            let remaining_ms = end_time - current_time;
            let remaining_sec = remaining_ms / 1000;

            let minutes = remaining_sec / 60;
            let seconds = remaining_sec % 60;
            format!("Round: {}:{:02}", minutes, seconds)
        }
    } else {
        "Round Over".to_string()
    };

    let mut timer_text = Text::new(&timer_str, &JERSEY_20);
    timer_text = timer_text.with_scale(0.025, ScaleMode::HeightRelative);

    let mut timer_text_bounds = timer_text.get_bounds_anchored_at(canvas_bounds.bottom_left().into(), Anchor::BottomLeft);
    timer_text_bounds = timer_text_bounds.translate_by_fraction(0.01, -0.03);

    let text_color = if let Some(end_time) = round_end_time_ms {
        let current_time = time::now();
        if current_time >= end_time {
            0xFF0000FF // Red for "Round Over"
        } else {
            let remaining_ms = end_time - current_time;
            if remaining_ms < 30000 { 0xFF0000FF } else { 0xFFFFFFFF }
        }
    } else {
        0xFF0000FF // Red for "Round Over"
    };

    timer_text.render(timer_text_bounds.xy().into(), text_color, true);
}

pub fn draw_round_over_timer(round_over_end_time_ms: Option<u64>) {
    let canvas_bounds = bounds::screen();

    let timer_str = if let Some(end_time) = round_over_end_time_ms {
        let current_time = time::now();
        if current_time >= end_time {
            "Starting...".to_string()
        } else {
            let remaining_ms = end_time - current_time;
            let remaining_sec = remaining_ms / 1000;
            format!("Next round in: {}s", remaining_sec + 1)
        }
    } else {
        "Starting...".to_string()
    };

    let mut timer_text = Text::new(&timer_str, &JERSEY_20);
    timer_text = timer_text.with_scale(0.03, ScaleMode::HeightRelative);

    let mut timer_text_bounds = timer_text.get_bounds_anchored_at(canvas_bounds.center().into(), Anchor::Center);
    timer_text_bounds = timer_text_bounds.translate_by_fraction(0.0, 0.3);

    let text_color = 0xFFFFFFFF; // White text
    timer_text.render(timer_text_bounds.xy().into(), text_color, true);
}
