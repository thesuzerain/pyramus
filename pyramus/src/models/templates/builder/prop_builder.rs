use crate::models::templates::prop::Prop;

use super::{BuilderType, ItemBuilder};

/// Builder for creating a prop
#[derive(Debug)]
pub enum PropTypeBuilder {
    /// Prop with random items within, varying on size, etc.
    /// Can be created with `ItemBuilder::build_random_prop`
    Random { width: u32, height: u32 },
    /// Empty prop
    Empty { width: u32, height: u32 },
}

impl PropTypeBuilder {
    pub fn build(self, name: String) -> Prop {
        match self {
            PropTypeBuilder::Random { width, height } => Prop::build_random(name, width, height),
            PropTypeBuilder::Empty { width, height } => Prop::new(name, width, height),
        }
    }
}

// TODO:             PropTypeBuilder::Random => Prop::build_random(name, width, height),

// TODO I don't like these being in ItemBuilder

impl ItemBuilder {
    /// Create a new prop with random items within, varying on size, etc
    pub fn build_random_prop(width: u32, height: u32) -> ItemBuilder {
        ItemBuilder {
            name: "random".to_string(),
            builder: BuilderType::Prop(PropTypeBuilder::Random { width, height }),
            parent: None,
            transform: Default::default(),
        }
    }
}
