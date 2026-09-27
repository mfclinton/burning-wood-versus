use turbo::*;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn time_based_lerp_factor(lerp_speed: f32, delta_time: f32) -> f32 {
    1.0 - (1.0 - lerp_speed).powf(delta_time)
}

pub fn lerp_with_easing<F>(start: f32, end: f32, t: f32, easing_fn: F) -> f32 
where F: Fn(f32) -> f32,
{
    let eased_t = easing_fn(t);
    lerp(start, end, eased_t)
}

pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    if a == b { 0.0 } 
    else { ((v - a) / (b - a)).clamp(0.0, 1.0) }
}

pub fn weighted_random_sample<T, F>(items: &[T], weight_fn: F) -> Option<T>
where 
    T: Clone,
    F: Fn(&T) -> f32,
{
    if items.is_empty() {
        return None;
    }

    let total_weight: f32 = items.iter().map(&weight_fn).sum();
    if total_weight <= 0.0 {
        return Some(items[0].clone());
    }
    
    let mut random_value = random::f32() * total_weight;
    for item in items {
        let weight = weight_fn(item);
        if random_value <= weight {
            return Some(item.clone());
        }
        random_value -= weight;
    }
    
    Some(items[0].clone())
}
