use std::collections::HashMap;

use crate::models::{editor::base_item::Base, templates::ids::PyramusId};


#[derive(Debug)]
pub struct Cache {
    pub cache : HashMap<PyramusId, CacheItem>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            cache: HashMap::new(),
        }
    }

    // TODO: This should actually fetch data from the internet (or some similar solution). Currently, it's just for testing.
    pub async fn fetch(&mut self) {
        let test_id = PyramusId(111);
        let test_str = include_str!("../../res/1n.json");
        let test_base: Base = serde_json::from_str(test_str).unwrap();
        self.cache.insert(test_id, CacheItem::Base(test_base));        
    }

    pub fn get_base(&self, id: PyramusId) -> Option<&Base> {
        match self.cache.get(&id) {
            Some(CacheItem::Base(base)) => Some(base),
            // TODO: maybe this should give an error, or we should do a generic get() function
            // Some(_) => None,
            None => None,
        }
    }

    pub fn insert_base(&mut self, id: PyramusId, base: Base) {
        self.cache.insert(id, CacheItem::Base(base));
    }
}

#[derive(Debug)]
pub enum CacheItem {
    Base(Base),
    // TODO: uploaded images should be here as well
}

impl From<Base> for CacheItem {
    fn from(base: Base) -> Self {
        CacheItem::Base(base)
    }
}
