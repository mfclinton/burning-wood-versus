use turbo::*;

#[derive(Debug)]
pub struct Font {
    pub name: &'static str,
}

impl Font {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
        }
    }
}
