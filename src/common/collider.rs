use turbo::*;

use crate::Vec2;

// --- Circle Collider ---

#[derive(Default)]
#[turbo::serialize]
pub struct CircleCollider {
    pub position: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
}

impl CircleCollider {
    pub fn new(position: Vec2, radius: f32) -> Self {
        CircleCollider { 
            position, 
            radius,
            velocity: Vec2::zero(),
        }
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }
}

// --- Box Collider ---

#[turbo::serialize]
pub struct BoxCollider {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
}

impl BoxCollider {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        BoxCollider { 
            position, 
            size,
            velocity: Vec2::zero(),
        }
    }
}

// --- Collision Detection Functions ---

// TODO:
pub fn circle_circle_collision(circle1: &CircleCollider, circle2: &CircleCollider) -> bool {
    let distance = circle1.position.distance_to(&circle2.position);
    distance < (circle1.radius + circle2.radius)
}

pub fn box_box_collision(box1: &BoxCollider, box2: &BoxCollider) -> bool {
    let half_1 = box1.size / 2.0;
    let half_2 = box2.size / 2.0;

    (box1.position - box2.position).abs() < (half_1 + half_2)
}

pub fn circle_box_collision(circle: &CircleCollider, box_collider: &BoxCollider) -> bool {
    let half_size = box_collider.size / 2.0;
    let closest = (circle.position - box_collider.position).clamp(-half_size.x, half_size.x) + box_collider.position;

    let distance = circle.position.distance_to(&closest);
    distance < circle.radius
}

// --- Rendering ---

impl CircleCollider {
    pub fn render_circle_collider(&self, color: u32) {
        circ!(
            d = self.diameter(),
            x = self.position.x - self.radius,
            y = self.position.y - self.radius,
            color = color
        );
    }
    
}
