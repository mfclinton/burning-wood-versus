
use turbo::*;

#[derive(PartialEq)]
#[turbo::serialize]
pub enum EventType {
    GasSurge,
    MeteorShower,
}
