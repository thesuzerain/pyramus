pub struct StagedItemId(u64);

pub struct Stage {
    pub items : Vec<StagedItem>,
}

pub struct StagedItem {
    pub id: StagedItemId,
    pub item: Item,
    
    pub parent: Option<StagedItemId>, // If None, then it's a root item
    pub relative_position: (f64, f64),
    pub relative_scale: (f64, f64),
    pub relative_rotation: f64, // In degrees
}

pub enum Item {
    Text(ItemText),
    Image(ItemImage),
}

pub struct ItemText {
    pub text: String,
}

pub struct ItemImage {
    pub url: String,
}

