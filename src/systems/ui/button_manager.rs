use turbo::*;

#[turbo::serialize]
pub struct ButtonManager {
    pub selected_index: usize,
    pub time_selected_last_pressed: u64,
    pub locked: bool,
    prev_selected_index: usize,
}

impl ButtonManager {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            time_selected_last_pressed: 0,
            locked: false,
            prev_selected_index: 0,
        }
    }

    pub fn update(&mut self, button_bounds: &[Bounds]) {
        if self.locked {
            return;
        }

        // Store previous selection
        self.prev_selected_index = self.selected_index;

        // Input Handling
        self.handle_mouse_navigation(button_bounds);
        self.handle_keyboard_navigation(button_bounds);
    }

    pub fn selection_changed(&self) -> bool {
        self.selected_index != self.prev_selected_index
    }

    pub fn selected_is_pressed(&mut self, button_bounds: &[Bounds]) -> bool {
        let mouse = mouse::screen();
        let keyboard = keyboard::get();

        // Mouse
        if mouse.intersects_bounds(button_bounds[self.selected_index]) && mouse.left.just_pressed() {
            self.time_selected_last_pressed = time::now();
            return true;
        }

        // Keyboard
        let key_pressed = keyboard.space().just_pressed() || keyboard.enter().just_pressed();
        if key_pressed {
            self.time_selected_last_pressed = time::now();
            return true;
        }
        
        false
    }

    // --- Navigation ---

    fn handle_mouse_navigation(&mut self, button_bounds: &[Bounds]) {
        let mouse = mouse::screen();

        for (index, bounds) in button_bounds.iter().enumerate() {
            if mouse.intersects_bounds(*bounds) {
                self.selected_index = index;
                break;
            }
        }
    }

    fn handle_keyboard_navigation(&mut self, button_bounds: &[Bounds]) {
        let keyboard = keyboard::get();
        
        let mut delta: i32 = 0;
        if keyboard.arrow_up().just_pressed() || keyboard.key_w().just_pressed() {
            delta = -1;
        }
        else if keyboard.arrow_down().just_pressed() || keyboard.key_s().just_pressed() {
            delta = 1;
        }

        self.selected_index = (self.selected_index as i32 + delta).clamp(0, button_bounds.len() as i32 - 1) as usize;
    }
}
