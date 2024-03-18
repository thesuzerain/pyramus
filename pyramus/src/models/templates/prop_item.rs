use std::{rc::Rc, sync::Arc};

use resvg::usvg::{self, NonZeroPositiveF32};

use super::{ids::ItemId, transform::RelativeTransform};

#[derive(Debug, Clone)] // TODO: remove clone
pub struct PropItem {
    pub id: ItemId,
    pub name: String,
    pub item: PropItemType,

    // Stageable objects (TODO: Move to a separate struct?)
    // Within a blueprint, if applicable, or parent
    pub parent: Option<ItemId>,
    pub children: Vec<ItemId>,        // If None, then it's a root item
    pub transform: RelativeTransform, // Within a prop (or parent)
}

impl PropItem {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        self.item.get_local_bounds()
    }

    /// Returns the size of the item
    pub fn get_size(&self) -> (u32, u32) {
        let (x0, y0, x1, y1) = self.get_local_bounds();
        ((x1 - x0) as u32, (y1 - y0) as u32)
    }
}

#[derive(Debug, Clone)] // TODO: remove clone
pub enum PropItemType {
    Image(PropItemImage),
    Text(PropItemText),
}

impl From<PropItemImage> for PropItemType {
    fn from(item: PropItemImage) -> Self {
        PropItemType::Image(item)
    }
}
impl From<PropItemText> for PropItemType {
    fn from(item: PropItemText) -> Self {
        PropItemType::Text(item)
    }
}

impl PropItemType {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        match self {
            // TODO: Text bounds
            PropItemType::Text(_) => (0.0, 0.0, 0.0, 0.0),
            PropItemType::Image(image) => (0.0, 0.0, image.viewport_width, image.viewport_height),
        }
    }
}

#[derive(Debug, Clone)] // TODO: remove clone
pub struct PropItemText {
    pub text: String,
    pub font_family: String,
    pub font_size: NonZeroPositiveF32,
    pub color: (u8, u8, u8),
    pub italic: bool,
}

#[derive(Debug, Clone)] // TODO: remove clone
pub struct PropItemImage {
    pub data: PropItemImageData,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

#[derive(Debug, Clone)]
pub enum PropItemImageData {
    Png(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Svg(Rc<usvg::Tree>),
}

impl From<PropItemImageData> for usvg::ImageKind {
    fn from(data: PropItemImageData) -> Self {
        match data {
            PropItemImageData::Png(data) => usvg::ImageKind::PNG(data.clone()),
            PropItemImageData::Jpeg(data) => usvg::ImageKind::JPEG(data.clone()),
            PropItemImageData::Gif(data) => usvg::ImageKind::GIF(data.clone()),
            PropItemImageData::Svg(data) => usvg::ImageKind::SVG((*data).clone()),
        }
    }
}
