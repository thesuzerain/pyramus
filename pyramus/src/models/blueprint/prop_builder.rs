use image::io::Reader as ImageReader;
use resvg::usvg::{self, NonZeroPositiveF32};
use std::{io::Cursor, rc::Rc, sync::Arc};

use crate::models::blueprint::prop::{PropItemImage, PropItemText};

use super::prop::{Prop, PropItemImageData, PropItemType};

pub enum PropItemBuilder {
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

impl PropItemBuilder {
    pub fn build_prop(self) -> crate::Result<Prop> {
        // TODO: Implement direct creation of Prop from PropItemBuilder
        todo!()
    }

    pub fn build(self) -> crate::Result<PropItemType> {
        match self {
            PropItemBuilder::Text {
                text,
                font_family,
                font_size,
                color,
                italic,
            } => Ok(PropItemType::Text(PropItemText {
                text,
                font_family,
                font_size: NonZeroPositiveF32::new(font_size).ok_or_else(|| {
                    crate::PyramusError::OtherError("Font size must be greater than 0".to_string())
                })?,
                color,
                italic,
            })),
            PropItemBuilder::ImageFromSvg(svg) => {
                let image = PropItemImage::from_svg_string(&svg)?;
                Ok(PropItemType::Image(image))
            }
            PropItemBuilder::ImageFromBytes { bytes, ext } => {
                let image = PropItemImage::from_bytes(bytes, &ext)?.ok_or_else(|| {
                    crate::PyramusError::OtherError("Could not parse image".to_string())
                })?;
                Ok(PropItemType::Image(image))
            }
        }
    }
}

impl PropItemImage {
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

    // Creates a simple SVG tree with a rectangle
    // TODO: This is for testing purposes only
    // Alpha is a value between 0.0 and 1.0
    pub fn from_rect(
        w: u32,
        h: u32,
        color: &str,
        stroke: Option<u32>,
        alpha: f32,
    ) -> Result<PropItemImage, usvg::Error> {
        let stroke = match stroke {
            Some(stroke) => format!(
                r#"stroke="{color}" stroke-width="{stroke}" fill="{color}" fill-opacity="{alpha}"#,
            ),
            None => format!(r#"fill="{color}" fill-opacity="{alpha}"#),
        };

        Self::from_svg_string(&format!(
            r#"
            <svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
                <rect x="0" y="0" width="{w}" height="{h}" {stroke}" />
            </svg>
            "#
        ))
    }
}

impl PropItemText {
    // TODO: 'Builder' pattern
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
