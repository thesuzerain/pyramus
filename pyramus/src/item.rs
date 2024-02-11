use js_sys::Math::random;
use resvg::usvg::{self, NonZeroPositiveF32};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StagedItemId(pub u32);

impl StagedItemId {
    pub fn new() -> StagedItemId {
        // TODO: Generate a unique ID
        let random_id = random() * u32::MAX as f64;
        StagedItemId(random_id as u32)
    }
}

#[derive(Debug)]
pub struct StagedItem {
    pub id: StagedItemId,
    pub name: String,

    pub item: Item,

    // TODO: SlotMap/Arena
    // TODO: Should these be Weak<> or Arc<>- a direct reference to the parent?
    pub parent: Option<StagedItemId>,
    pub children: Vec<StagedItemId>, // If None, then it's a root item

    pub transform: RelativeTransform,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ItemText {
    pub text: String,
    pub font_family: String,
    pub font_size: NonZeroPositiveF32,
    pub color: (u8, u8, u8),
    pub italic: bool,
}

impl ItemText {
    // TODO: 'Builder' pattern
    pub fn build(text: String) -> ItemText {
        ItemText {
            text,
            font_family: "Arial".to_string(),
            font_size: NonZeroPositiveF32::new(12.0).expect("12.0 is not a NonZeroPositiveF32"),
            color: (255, 255, 255), // White
            italic: false,
        }
    }
}

#[derive(Debug)]
pub struct ItemImage {
    pub data: ItemImageData,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

#[derive(Debug)]
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
                <rect x="0" y="0" width="{w}" height="{h}" fill="{bg}" fill-opacity="{alpha}" />
            </svg>
            "#
        ))
    }
}
