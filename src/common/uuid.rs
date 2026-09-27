use turbo::*;

pub fn create_uuid(tick: u64) -> String {
    format!("uuid-{}-{}", tick, random::u64())
}
