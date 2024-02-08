use std::sync::{Arc, RwLock, Weak};

use resvg::usvg::{self};

pub struct Stage {
    pub size: (u32, u32),
    // TODO: It might be better to use a SlotMap/Arena here, rather than just tree structure
    pub root: Arc<RwLock<StagedItem>>,
}

impl Stage {
    // TODO: Background color should be a color type (consistency with other parts of the codebase)
    // TODO: Background color should be optional, or a pattern (like how Photoshop does transparency)
    pub fn build(width: u32, height: u32) -> Stage {
        let root = StagedItem {
            item: Item::Image(ItemImage::from_rect(width, height, "red", 1.0).unwrap()),
            children: Vec::new(),
            parent: None,
            transform: RelativeTransform::default(),
        };

        Stage {
            size: (width, height),
            root: Arc::new(RwLock::new(root)),
        }
    }

    // Adds item to root
    pub fn add_child(
        &mut self,
        item: Item,
        transform: Option<RelativeTransform>,
    ) -> Arc<RwLock<StagedItem>> {
        StagedItem::add_child(self.root.clone(), item, transform)
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::build(800, 600)
    }
}

pub struct StagedItem {
    pub item: Item,

    // TODO: SlotMap/Arena
    pub parent: Option<Weak<RwLock<StagedItem>>>,
    pub children: Vec<Arc<RwLock<StagedItem>>>, // If None, then it's a root item

    pub transform: RelativeTransform,
}

pub struct RelativeTransform {
    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub rotation: f32, // In degrees
}

impl Default for RelativeTransform {
    fn default() -> Self {
        RelativeTransform {
            position: (0.0, 0.0),
            scale: (1.0, 1.0),
            rotation: 0.0,
        }
    }
}

impl StagedItem {
    // TODO: This pattern could be improved (taking in an Arc<RwLock> parent, rather than a reference to self)
    pub fn add_child(
        parent: Arc<RwLock<StagedItem>>,
        item: Item,
        transform: Option<RelativeTransform>,
    ) -> Arc<RwLock<StagedItem>> {
        let item = StagedItem {
            item,
            children: Vec::new(),
            parent: Some(Arc::downgrade(&parent)),
            transform: transform.unwrap_or_default(),
        };
        let item = Arc::new(RwLock::new(item));
        {
            let mut parent = parent.write().unwrap();
            parent.children.push(item.clone());
        }
        item
    }
}

pub enum Item {
    Text(ItemText),
    Image(ItemImage),
}

impl From<ItemImage> for Item {
    fn from(image: ItemImage) -> Self {
        Item::Image(image)
    }
}

impl From<ItemText> for Item {
    fn from(text: ItemText) -> Self {
        Item::Text(text)
    }
}

pub struct ItemText {
    pub text: String,
    pub font_family: String,
    pub font_size: f32,
    pub color: (u8, u8, u8),
    pub italic: bool,
}

impl ItemText {
    // TODO: 'Builder' pattern
    pub fn build(text: String) -> ItemText {
        ItemText {
            text,
            font_family: "Arial".to_string(),
            font_size: 12.0,
            color: (255, 255, 255), // White
            italic: false,
        }
    }
}

pub struct ItemImage {
    pub data: ItemImageData,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

pub enum ItemImageData {
    Png(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Svg(Arc<usvg::Tree>),
}

impl ItemImage {
    pub fn from_bytes(
        bytes: Vec<u8>,
        viewport_width: f32,
        viewport_height: f32,
        ext: &str,
    ) -> Option<ItemImage> {
        let data = match ext {
            "png" => Some(ItemImageData::Png(Arc::new(bytes))),
            "jpg" | "jpeg" => Some(ItemImageData::Jpeg(Arc::new(bytes))),
            "gif" => Some(ItemImageData::Gif(Arc::new(bytes))),
            "svg" => {
                let tree = usvg::Tree::from_data(&bytes, &usvg::Options::default()).ok()?;
                Some(ItemImageData::Svg(Arc::new(tree)))
            }
            _ => None,
        };
        data.map(|data| ItemImage {
            data,
            viewport_width,
            viewport_height,
        })
    }

    pub fn from_svg_string(svg: &str) -> Result<ItemImage, usvg::Error> {
        let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default())?;
        let tree_height = tree.size.height();
        let tree_width = tree.size.width();
        Ok(ItemImage {
            data: ItemImageData::Svg(Arc::new(tree)),
            viewport_width: tree_width,
            viewport_height: tree_height,
        })
    }

    // Creates a simple SVG tree with a rectangle
    // TODO: This is for testing purposes only
    // Alpha is a value between 0.0 and 1.0
    pub fn from_rect(w: u32, h: u32, bg: &str, alpha: f32) -> Result<ItemImage, usvg::Error> {
        Self::from_svg_string(&format!(
            r#"
            <svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
                <rect x="10" y="10" width="{w}" height="{h}" fill="{bg}" fill-opacity="{alpha}" />
            </svg>
            "#
        ))
    }
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage() -> Stage {
    let mut stage = Stage::default();

    // Add a simple translucent rectangle as the background
    stage.add_child(
        ItemImage::from_rect(300, 200, "blue", 0.5).unwrap().into(),
        None,
    );

    // // Add example text and image
    StagedItem::add_child(
        stage.root.clone(),
        Item::Image(ItemImage {
            viewport_height: 200.0,
            viewport_width: 300.0,
            data: ItemImageData::Jpeg(include_bytes!("../../testimg.jpg").to_vec().into()),
        }),
        Some(RelativeTransform {
            position: (50.0, 50.0),
            scale: (0.5, 0.5),
            rotation: 45.0,
        }),
    );

    StagedItem::add_child(
        stage.root.clone(),
        Item::Text(ItemText::build("Hello, world!".to_string())),
        Some(RelativeTransform {
            position: (0.0, 0.0),
            scale: (1.0, 3.0),
            rotation: -10.0,
        }),
    );
    stage
}
