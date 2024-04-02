use std::{rc::Rc, sync::Arc};

use resvg::usvg::{self, NonZeroPositiveF32};
use serde::{Deserialize, Serialize};

use crate::models::editor::staging::StagingContext;

use super::ids::ItemId;

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO: remove clone
pub struct PropItem {
    pub id: ItemId,
    pub name: String,
    pub item: PropItemType,

    pub staging: StagingContext,
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

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO: remove clone
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

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO: remove clone
pub struct PropItemText {
    pub text: String,
    pub font_family: String,
    #[serde(deserialize_with = "deserialize_nonzero_f32")]
    #[serde(serialize_with = "serialize_nonzero_f32")]
    pub font_size: NonZeroPositiveF32,
    pub color: (u8, u8, u8),
    pub italic: bool,
}

// TODO: move these later.
// TODO: Also, is there a beter way?
fn deserialize_nonzero_f32<'de, D>(deserializer: D) -> Result<NonZeroPositiveF32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = f32::deserialize(deserializer)?;
    NonZeroPositiveF32::new(f)
        .ok_or_else(|| serde::de::Error::custom("Font size must be greater than 0"))
}
fn serialize_nonzero_f32<S>(f: &NonZeroPositiveF32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    f.get().serialize(serializer)
}

fn deserialize_rc_usvg_tree<'de, D>(deserializer: D) -> Result<Rc<usvg::Tree>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data = String::deserialize(deserializer)?;
    let tree =
        usvg::Tree::from_str(&data, &usvg::Options::default()).map_err(serde::de::Error::custom)?;
    Ok(Rc::new(tree))
}

fn serialize_rc_usvg_tree<S>(tree: &Rc<usvg::Tree>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data = tree.to_string(&usvg::XmlOptions::default());
    data.serialize(serializer)
}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO: remove clone
pub struct PropItemImage {
    pub data: PropItemImageData,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropItemImageData {
    Png(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    #[serde(deserialize_with = "deserialize_rc_usvg_tree")]
    #[serde(serialize_with = "serialize_rc_usvg_tree")]
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
