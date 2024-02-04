use crate::{log, models::{Item, ItemImage, Stage, StagedItem}};
use resvg::{tiny_skia, usvg::{self, NodeExt, Transform, TreeWriting, XmlOptions}};
use wasm_bindgen::{Clamped, JsCast, JsValue};
use std::ffi::OsStr;
use usvg::{TreeParsing};
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

impl Stage {
    pub fn to_usvg_tree(&self) -> usvg::Tree {
        let root_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));
		let tree = usvg::Tree {
			size: usvg::Size::from_wh(self.size.0 as f32, self.size.1 as f32).unwrap(),
			view_box: usvg::ViewBox {
				rect: usvg::NonZeroRect::from_xywh(0., 0., 1., 1.).unwrap(),
				aspect: usvg::AspectRatio::default(),
			},
			root: root_node.clone(),
		};

		for element in self.items.iter() {
			root_node.append(element.to_usvg_node());
		}
		tree
    }
}

impl StagedItem {
    pub fn to_usvg_node(&self) -> usvg::Node {
        let transform = usvg::Transform::from_translate(self.relative_position.0, self.relative_position.1).post_scale(self.relative_scale.0, self.relative_scale.1).post_rotate(self.relative_rotation);
        let node = self.item.to_usvg_node(transform);
        node
    }
}

impl Item {
    pub fn to_usvg_node(&self, transform : usvg::Transform) -> usvg::Node {
        match &self {
            Item::Text(text) => {
                usvg::Node::new(usvg::NodeKind::Text(usvg::Text {
                    id: String::new(),
                    transform,
                    positions: Vec::new(),
                    rendering_mode: usvg::TextRendering::OptimizeSpeed,
                    rotate: vec![],
                    writing_mode: usvg::WritingMode::LeftToRight,
                    chunks: Vec::new(),

                }))
            },
            Item::Image(image) => {
                usvg::Node::new(usvg::NodeKind::Image(usvg::Image {
                    id: String::new(),
                    transform,
                    visibility: usvg::Visibility::Visible,
                    view_box: usvg::ViewBox {
                        rect: usvg::NonZeroRect::from_xywh(0., 0., 1., 1.).unwrap(),
                        aspect: usvg::AspectRatio::default(),
                    },
                    rendering_mode: usvg::ImageRendering::OptimizeSpeed,
                    kind: match image {
                        ItemImage::Png(data) => usvg::ImageKind::PNG(data.clone()),
                        ItemImage::Jpeg(data) => usvg::ImageKind::JPEG(data.clone()),
                        ItemImage::Gif(data) => usvg::ImageKind::GIF(data.clone()),
                    }
                }))
            }
        }
    }
}

// example_stage

pub fn render(stage: &Stage, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();


    let options = usvg::Options::default();
    let tree = stage.to_usvg_tree();
    log::log("Made tree");
    let resvg_tree = resvg::Tree::from_usvg(&tree);

    // pixmap_size = pixmap_size.scale_to_width(pixmap_size.width() * 80). unwrap();
    let tree_size = resvg_tree.size.to_int_size();

    let width_scale = canvas_width as f32 / tree_size.width() as f32;
    let height_scale = canvas_width as f32 / tree_size.height() as f32;

    let min_scale = width_scale.min(height_scale);
    log::log(&format!("Width scale: {}", width_scale));
    log::log(&format!("Height scale: {}", height_scale));

    let transform = Transform::from_scale(min_scale, min_scale);

    let mut pixmap = tiny_skia::Pixmap::new(canvas_width, canvas_height).unwrap();
    log::log("Made pixmap");
    
    resvg_tree.render(transform, &mut pixmap.as_mut());
    log::log(&format!("Pixmap size: {:?}", tree_size));
    
    let array: Clamped<&[u8]> = Clamped(pixmap.data());
    log::log(format!("Sizes: {} {}", tree_size.width(), tree_size.height()).as_str());

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(array, canvas_width, canvas_height)?;
    
    log::log(&format!("Image data: {:?}", image_data));
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    log::log("Put image data");

    Ok(())
}

pub fn render_string(stage: &Stage, canvas: &HtmlCanvasElement) -> Result<String, JsValue> {
    let tree = stage.to_usvg_tree();
    let s = tree.to_string(&XmlOptions::default());
    log::log(&format!("Tree: {}", s));
    Ok(s)
}

// pub fn render(stage: &Stage, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
//     let canvas_width = canvas.width();
//     let canvas_height = canvas.height();


//     let example = include_str!("../../pyramus-gui/src/assets/external/discord.svg");
//     log::log(&format!("Example: {}", example));

//     let options = usvg::Options::default();
//     let tree = usvg::Tree::from_str(example, &options).unwrap();
//     log::log("Made tree");
//     let resvg_tree = resvg::Tree::from_usvg(&tree);

//     // pixmap_size = pixmap_size.scale_to_width(pixmap_size.width() * 80). unwrap();
//     let tree_size = resvg_tree.size.to_int_size();

//     let width_scale = canvas_width as f32 / tree_size.width() as f32;
//     let height_scale = canvas_width as f32 / tree_size.height() as f32;

//     let min_scale = width_scale.min(height_scale);
//     log::log(&format!("Width scale: {}", width_scale));
//     log::log(&format!("Height scale: {}", height_scale));

//     let transform = Transform::from_scale(min_scale, min_scale);

//     let mut pixmap = tiny_skia::Pixmap::new(canvas_width, canvas_height).unwrap();
//     log::log("Made pixmap");
    
//     resvg_tree.render(transform, &mut pixmap.as_mut());
//     log::log(&format!("Pixmap size: {:?}", tree_size));
    
//     let array: Clamped<&[u8]> = Clamped(pixmap.data());
//     log::log(format!("Sizes: {} {}", tree_size.width(), tree_size.height()).as_str());

//     let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(array, canvas_width, canvas_height)?;
    
//     log::log(&format!("Image data: {:?}", image_data));
//     context.put_image_data(&image_data, 0.0, 0.0).unwrap();
//     log::log("Put image data");

//     Ok(())
// }
