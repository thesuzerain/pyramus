use std::str::FromStr;

use js_sys::Math::random;
use serde::{Deserialize, Serialize};
use base62::{encode, decode};

/// PyramusId is a unique identifier across the pyramus system.
/// These are generated externally, and are used to describe shareable objects.
/// (e.g. downloading a prop from the internet)
/// It is serialized/deserialized through conversion to base58
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyramusId(pub u32);

impl PyramusId {

    // TODO: This is purely a debug function before the external server is set up.
    // TODO: Delete this when the external server is set up.
    pub fn debug_new() -> PyramusId {
        let random_id = random() * u32::MAX as f64;
        PyramusId(random_id as u32)
    }
}

impl Serialize for PyramusId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        encode(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PyramusId {
    fn deserialize<D>(deserializer: D) -> Result<PyramusId, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = decode(s).map_err(serde::de::Error::custom)?;
        Ok(PyramusId(s as u32))
    }
}

impl FromStr for PyramusId {
    type Err = base62::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = decode(s)?;
        Ok(PyramusId(s as u32))
    }
}



/// InternalId is a unique identifier for an item within any any private context.
/// Two different props may have items with the same internal ID, but two items within the same prop will not.
/// (No collision checking is done)
/// TODO: Checks to ensure uniqueness
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InternalId(pub u32);
impl InternalId {
    pub fn new() -> InternalId {
        // TODO: Generate a unique ID
        let random_id = random() * u32::MAX as f64;
        InternalId(random_id as u32)
    }
}
impl Default for InternalId {
    fn default() -> Self {
        Self::new()
    }
}
