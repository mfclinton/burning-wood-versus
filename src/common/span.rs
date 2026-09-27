use crate::{
    inverse_lerp, lerp_with_easing
};

#[derive(Copy, Clone)]
pub struct Span { start: f32, end: f32 }

impl Span {
    pub const fn new(start: f32, end: f32) -> Self { Self { start, end } }

    pub fn value(self, t: f32, from: f32, to: f32, easing: fn(f32) -> f32) -> f32 {
        lerp_with_easing(from, to, self.local_t(t), easing)
    }

    fn local_t(self, t: f32) -> f32 {
        inverse_lerp(self.start, self.end, t).clamp(0.0, 1.0)
    }
}
