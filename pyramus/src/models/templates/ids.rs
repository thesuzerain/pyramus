use js_sys::Math::random;
use serde::{Deserialize, Serialize};

/// ItemId is a unique identifier for an item within any context.
/// TODO: Checks to ensure uniqueness
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemId(pub u32);
impl ItemId {
    pub fn new() -> ItemId {
        // TODO: Generate a unique ID
        let random_id = random() * u32::MAX as f64;
        ItemId(random_id as u32)
    }
}
impl Default for ItemId {
    fn default() -> Self {
        Self::new()
    }
}
