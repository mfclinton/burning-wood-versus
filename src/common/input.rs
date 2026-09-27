use turbo::*;

use crate::{Vec2, CircularPad};

const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%&-_<>?";

// --- Movement Input ---

pub fn movement_vector() -> Vec2 {
    let mut mov = Vec2::zero();

    if gamepad::get(0).up.pressed() {
        mov.y -= 1.0;
    }
    if gamepad::get(0).down.pressed() {
        mov.y += 1.0;
    }
    if gamepad::get(0).left.pressed() {
        mov.x -= 1.0;
    }
    if gamepad::get(0).right.pressed() {
        mov.x += 1.0;
    }

    mov.normalized()
}

pub fn movement_vector_with_pad(circular_pad: &CircularPad) -> Vec2 {
    let gamepad_input = movement_vector();
    let pad_input = circular_pad.get_movement_vector();
    
    if pad_input.length() > 0.01 {
        pad_input
    } else {
        gamepad_input
    }
}

// --- Keyboard ---

pub fn handle_text_input(text: &mut String, max_length: usize) -> bool {
    let keyboard = keyboard::get();
    let mut modified = false;

    // Remove
    if (keyboard.backspace().just_pressed() || keyboard.delete().just_pressed()) && !text.is_empty() {
        text.pop();
        modified = true;
    }

    // Add
    if text.len() < max_length {
        let new_char = keyboard.text();
        if ALLOWED_CHARS.contains(&new_char) {
            text.push_str(&new_char);
        }
    }

    modified
}
