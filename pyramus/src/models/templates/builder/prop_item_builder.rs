use crate::{
    models::templates::{
        ids::ItemId,
        prop_item::{PropItemImage, PropItemImageData, PropItemText, PropItemType},
        transform::RelativeTransform,
    },
    svg,
};
use image::io::Reader as ImageReader;
use resvg::usvg::{self, NonZeroPositiveF32};
use std::{io::Cursor, rc::Rc, sync::Arc};

use super::{BuilderType, ItemBuilder};

// TODO I don't like these being in ItemBuilder
impl ItemBuilder {
    /// Creates an ItemBuilder with a basic text item (12pt Arial, black, not italicized)
    pub fn build_text_basic(text: impl ToString) -> ItemBuilder {
        Self::build_text(text, "Arial".to_string(), 12.0, (0, 0, 0), false)
    }

    /// Creates an ItemBuilder with a text item
    pub fn build_text(
        text: impl ToString,
        font_family: String,
        font_size: f32,
        color: (u8, u8, u8),
        italic: bool,
    ) -> ItemBuilder {
        ItemBuilder {
            name: "text".to_string(),
            builder: BuilderType::PropItem(PropItemTypeBuilder::Text {
                text: text.to_string(),
                font_family,
                font_size,
                color,
                italic,
            }),
            parent: None,
            transform: Default::default(),
        }
    }

    /// Creates an ItemBuilder with an image item from an SVG string
    pub fn build_image_from_svg(svg: String) -> ItemBuilder {
        ItemBuilder {
            name: "image".to_string(),
            builder: BuilderType::PropItem(PropItemTypeBuilder::ImageFromSvg(svg)),
            parent: None,
            transform: Default::default(),
        }
    }

    /// Creates an ItemBuilder with an image item from bytes
    pub fn build_image_from_bytes(bytes: Vec<u8>, ext: impl ToString) -> ItemBuilder {
        ItemBuilder {
            name: "image".to_string(),
            builder: BuilderType::PropItem(PropItemTypeBuilder::ImageFromBytes {
                bytes,
                ext: ext.to_string(),
            }),
            parent: None,
            transform: Default::default(),
        }
    }

    /// Creates a simple SVG tree with a rectangle
    // TODO: This is for testing purposes only
    /// Alpha is a value between 0.0 and 1.0
    pub fn build_image_from_rect(
        w: u32,
        h: u32,
        color: &str,
        stroke: Option<u32>,
        alpha: f32,
    ) -> ItemBuilder {
        ItemBuilder::build_image_from_svg(svg::build_svg_rect(w, h, color, stroke, alpha))
    }

    /// Adds a parent to the ItemBuilder
    pub fn parent(mut self, parent: ItemId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Adds a transform to the ItemBuilder
    pub fn transform(mut self, transform: RelativeTransform) -> Self {
        self.transform = transform;
        self
    }

    /// Adds a name to the ItemBuilder
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

/// Builder for creating a prop item
#[derive(Debug)]
pub enum PropItemTypeBuilder {
    Text {
        text: String,
        font_family: String,
        font_size: f32,
        color: (u8, u8, u8),
        italic: bool,
    },
    ImageFromSvg(String),
    ImageFromBytes {
        bytes: Vec<u8>,
        ext: String,
    },
}

impl PropItemTypeBuilder {
    pub fn build(self) -> crate::Result<PropItemType> {
        Ok(match self {
            PropItemTypeBuilder::Text {
                text,
                font_family,
                font_size,
                color,
                italic,
            } => PropItemType::Text(PropItemText {
                text,
                font_family,
                font_size: NonZeroPositiveF32::new(font_size).ok_or_else(|| {
                    crate::PyramusError::OtherError("Font size must be greater than 0".to_string())
                })?,
                color,
                italic,
            }),
            PropItemTypeBuilder::ImageFromSvg(svg) => {
                let image = PropItemImage::from_svg_string(&svg)?;
                PropItemType::Image(image)
            }
            PropItemTypeBuilder::ImageFromBytes { bytes, ext } => {
                let image = PropItemImage::from_bytes(bytes, &ext)?.ok_or_else(|| {
                    crate::PyramusError::OtherError("Could not parse image".to_string())
                })?;
                PropItemType::Image(image)
            }
        })
    }
}

impl PropItemImage {
    /// Create a PropItemImage from bytes (guessing the format from the extension)
    pub fn from_bytes(
        bytes: Vec<u8>,
        ext: &str, // TODO: Might not need this with with_guessed_format
    ) -> crate::Result<Option<PropItemImage>> {
        // TODO: find a different way to get height/width (especially given that files will be dynamically uploaded- can get size through that)
        // TODO: This may only apply to right now when we are using include_bytes! to load the image
        // This is very awkward, but it seems like we cant get the bytes back out without rewriting it.
        // TODO: It might be good to just straight-up use DynamicImage in ItemImage
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        let viewport_width = img.width() as f32;
        let viewport_height = img.height() as f32;

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

        let data = match ext {
            "png" => Some(PropItemImageData::Png(Arc::new(bytes))),
            "jpg" | "jpeg" => Some(PropItemImageData::Jpeg(Arc::new(bytes))),
            "gif" => Some(PropItemImageData::Gif(Arc::new(bytes))),
            "svg" => {
                let tree = match usvg::Tree::from_data(&bytes, &usvg::Options::default()) {
                    Ok(tree) => tree,
                    Err(err) => {
                        // TODO: Should this be an error?
                        crate::log!("Could not parsing SVG: {err}");
                        return Ok(None);
                    }
                };
                Some(PropItemImageData::Svg(Rc::new(tree)))
            }
            _ => None,
        };
        Ok(data.map(|data| PropItemImage {
            data,
            viewport_width,
            viewport_height,
        }))
    }

    /// Create a PropItemImage from an SVG string
    pub fn from_svg_string(svg: &str) -> Result<PropItemImage, usvg::Error> {
        let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default())?;
        let tree_height = tree.size.height();
        let tree_width = tree.size.width();
        Ok(PropItemImage {
            data: PropItemImageData::Svg(Rc::new(tree)),
            viewport_width: tree_width,
            viewport_height: tree_height,
        })
    }
}

impl PropItemText {
    // TODO: 'Builder' pattern
    /// Finish building the PropItemText
    pub fn build(text: String) -> PropItemText {
        PropItemText {
            text,
            font_family: "Arial".to_string(),
            font_size: NonZeroPositiveF32::new(12.0).expect("12.0 is not a NonZeroPositiveF32"),
            color: (255, 255, 255), // White
            italic: false,
        }
    }
}
