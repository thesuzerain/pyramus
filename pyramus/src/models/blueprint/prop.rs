use std::{collections::HashMap, rc::Rc, sync::Arc};

use resvg::usvg::{self, NonZeroPositiveF32};

use super::{ids::ItemId, transform::RelativeTransform};

#[derive(Debug)]
pub struct Prop {
    pub name: String,

    pub items: HashMap<ItemId, PropItem>,
    pub root: ItemId,
    pub size: (u32, u32), // TODO: Should this crop to the total bounds of the items?
}

impl Prop {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        (0.0, 0.0, self.size.0 as f32, self.size.1 as f32)
    }
}

#[derive(Debug)]
pub struct PropItem {
    pub id: ItemId,
    pub name: String,
    pub item: PropItemType,
    pub parent: Option<ItemId>,
    pub children: Vec<ItemId>, // If None, then it's a root item

    pub transform: RelativeTransform,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct PropItemText {
    pub text: String,
    pub font_family: String,
    pub font_size: NonZeroPositiveF32,
    pub color: (u8, u8, u8),
    pub italic: bool,
}

#[derive(Debug)]
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
