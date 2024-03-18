use crate::models::templates::prop::Prop;

pub enum PropTypeBuilder {
    Random { width: u32, height: u32 },
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
