use glam::Vec2;
use js_sys::Math::random;
use resvg::usvg::{self, NonZeroPositiveF32};
use std::{io::Cursor, sync::Arc};
use image::io::Reader as ImageReader;
use super::stage::Stage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StagedItemId(pub u32);

impl Default for StagedItemId {
    fn default() -> Self {
        Self::new()
    }
}

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
    // TODO: Should these be Weak<> or Rc<>- a direct reference to the parent?
    pub parent: Option<StagedItemId>,
    pub children: Vec<StagedItemId>, // If None, then it's a root item

    pub transform: RelativeTransform,
}

impl StagedItem {
    pub fn contains_point(&self, x: f32, y: f32, stage : &Stage) -> bool {
        let transform = self.get_screen_transform(stage);
        let click = glam::Vec2::new(x, y);
        let click = transform.inverse().transform_point2(click);
        let (x0, y0, x1, y1) = self.item.get_local_bounds();
        click.x >= x0 && click.x <= x1 && click.y >= y0 && click.y <= y1
    }

    pub fn get_screen_transform(&self, stage : &Stage) -> glam::Affine2 {
        // TODO: If we add 3d, this needs a projection matrix/camera and world space as an intermediate step
        let transform = self.transform.to_glam_affine();
        if let Some(parent_id) = self.parent {
            let parent_item = stage.items.get(&parent_id).expect("Parent item not found");
            transform * parent_item.get_screen_transform(stage) 
        } else {
            transform
        }
    }

    // x0, y0, x1, y1
    pub fn get_bounds(&self, stage : &Stage) -> (f32, f32, f32, f32) {
        let (x0, y0, x1, y1) = self.item.get_local_bounds();

        let transform = self.get_screen_transform(stage);
        crate::log!("Transform for {name}: {transform}, onto {x0} {y0}, {x1} {y1}", name = self.name);
        let Vec2 { x: x0, y : y0} = transform.transform_point2(glam::Vec2::new(x0 , y0));
        let Vec2 { x: x1, y: y1} = transform.transform_point2(glam::Vec2::new(x1 , y1));

        (f32::min(x0,x1), f32::min(y0,y1), f32::max(x0,x1), f32::max(y0,y1))

    }
}

// TODO: Is this still needed now that we have Affine2?
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

// TODO: From?
impl RelativeTransform {
    pub fn to_glam_affine(&self) -> glam::Affine2 {
        let (x, y) = self.position;
        let (sx, sy) = self.scale;
        let r = self.rotation.to_radians();
        glam::Affine2::from_scale_angle_translation(glam::Vec2::new(sx, sy), r, glam::Vec2::new(x, y))
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

impl Item {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        match self {
            // TODO: Text bounds
            Item::Text(_) => (0.0, 0.0, 0.0, 0.0),
            Item::Image(image) => (0.0, 0.0, image.viewport_width, image.viewport_height),
        }
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

#[derive(Debug, Clone)]
pub enum ItemImageData {
    Png(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Svg(Arc<usvg::Tree>),
}

impl ItemImage {
    pub fn from_bytes(
        bytes: Vec<u8>,
        ext: &str, // TODO: Might not need this with with_guessed_format
    ) -> crate::Result<Option<ItemImage>> {
        // TODO: find a different way to get height/width (especially given that files will be dynamically uploaded- can get size through that)
        // TODO: This may only apply to right now when we are using include_bytes! to load the image
        // This is very awkward, but it seems like we cant get the bytes back out without rewriting it.
        let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
        let viewport_width = img.width() as f32;
        let viewport_height = img.height() as f32;

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        
        // TODO: It might be good to just straight-up use DynamicImage in ItemImage
        // let bytes = img.into_bytes().to_vec();
        let data = match ext {
            "png" => Some(ItemImageData::Png(Arc::new(bytes))),
            "jpg" | "jpeg" => Some(ItemImageData::Jpeg(Arc::new(bytes))),
            "gif" => Some(ItemImageData::Gif(Arc::new(bytes))),
            "svg" => {
                let tree = match usvg::Tree::from_data(&bytes, &usvg::Options::default()) {
                    Ok(tree) => tree,
                    Err(err) => {
                        // TODO: Should this be an error?
                        crate::log!("Could not parsing SVG: {err}");
                        return Ok(None);
                    }
                };
                Some(ItemImageData::Svg(Arc::new(tree)))
            }
            _ => None,
        };
        Ok(data.map(|data| ItemImage {
            data,
            viewport_width,
            viewport_height,
        }))
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
    pub fn from_rect(w: u32, h: u32, color: &str, stroke : Option<u32>, alpha: f32) -> Result<ItemImage, usvg::Error> {
        let stroke = match stroke {
            Some(stroke) => format!(r#"stroke="{color}" stroke-width="{stroke}" fill="{color}" fill-opacity="{alpha}"#,),
            None => format!(r#"fill="{color}" fill-opacity="{alpha}"#)
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


impl From<ItemImageData> for usvg::ImageKind {
    fn from(data: ItemImageData) -> Self {
        match data {
            ItemImageData::Png(data) => usvg::ImageKind::PNG(data.clone()),
            ItemImageData::Jpeg(data) => usvg::ImageKind::JPEG(data.clone()),
            ItemImageData::Gif(data) => usvg::ImageKind::GIF(data.clone()),
            ItemImageData::Svg(data) => usvg::ImageKind::SVG((*data).clone()),
        }
    }
}