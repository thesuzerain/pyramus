use crate::{
    log,
    models::{Item, ItemImageData, Stage, StagedItem},
};
use svgtypes::parse_font_families;

use resvg::{
    tiny_skia,
    usvg::{self, Font, FontStyle, NonZeroPositiveF32, TextSpan, Transform, XmlOptions},
};
use usvg::fontdb;

use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

impl Stage {
    pub fn to_usvg_tree(&self) -> usvg::Tree {
        let mut tree = usvg::Tree {
            size: usvg::Size::from_wh(self.size.0 as f32, self.size.1 as f32).unwrap(),
            view_box: usvg::ViewBox {
                // TODO: Look here for viewbox- will need to revisit this section as we do resizing
                rect: usvg::NonZeroRect::from_xywh(0., 0., self.size.0 as f32, self.size.1 as f32)
                    .unwrap(),
                aspect: usvg::AspectRatio::default(),
            },
            root: usvg::Group::default(),
        };

        // Recursively add children to the root node
        // TODO: A slotmap may improve this, as we no longer need to hold a lock on the root node
        let root = self.root.read().unwrap();
        {
            tree.root.children.push(root.to_usvg_node());
        }

        // Postprocessing step
        // TODO: We shouldnt be repeatedly creating the fontdb- should be in a state, or something
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();

        let steps = resvg::usvg::PostProcessingSteps {
            // `resvg` cannot render text as is. We have to convert it into paths first.
            convert_text_into_paths: true,
        };
        tree.postprocess(steps, &fontdb);

        tree
    }
}

impl StagedItem {
    pub fn to_usvg_node(&self) -> usvg::Node {
        // TODO: Transforming is not done yet- doesnt inheret from parents, and also scaling seems to move the object
        let transform =
            usvg::Transform::from_scale(self.transform.scale.0, self.transform.scale.1)
                .post_rotate(self.transform.rotation)
                .post_translate(self.transform.position.0, self.transform.position.1);

        // All nodes are contained in a group node, so we can apply the transform to the group node, and then apply the transform to the children nodes
        // TODO: Is this needed?
        let mut children = vec![self.item.to_usvg_node()];
        for child in &self.children {
            children.push(child.read().unwrap().to_usvg_node());
        }

        usvg::Node::Group(Box::new(usvg::Group {
            transform,
            children,
            ..Default::default()
        }))
    }
}

impl Item {
    pub fn to_usvg_node(&self) -> usvg::Node {
        match &self {
            Item::Text(text) => {
                // TODO: There doesn't seem to be a way in resvg to create a text node directly/simply.
                // An alternative would be simply parsing a string- but that's hacky, and it might reload fonts.
                // TODO: Check if it reloads fonts, and/or find a way to do this more simply.

                // TODO: pass fontdb or a config containing it through, or get from a state
                let mut fontdb = fontdb::Database::new();
                fontdb.load_system_fonts();

                // todo: shouldn't hardcode this
                let font_families = parse_font_families(&text.font_family).unwrap();

                let num_chars = text.text.chars().count();
                usvg::Node::Text(Box::new(usvg::Text {
                    id: String::new(),
                    dx: vec![0.0],
                    dy: vec![0.0],
                    rotate: vec![0.0],

                    abs_transform: Transform::identity(), // Set on postprocessing, not here
                    abs_bounding_box: None,
                    abs_stroke_bounding_box: None,
                    bounding_box: None,
                    stroke_bounding_box: None,
                    flattened: None,
                    rendering_mode: usvg::TextRendering::OptimizeSpeed,
                    writing_mode: usvg::WritingMode::LeftToRight,
                    chunks: vec![usvg::TextChunk {
                        text: text.text.to_string(),
                        x: None,
                        y: None,
                        anchor: usvg::TextAnchor::Middle,
                        spans: vec![TextSpan {
                            start: 0,
                            end: num_chars,
                            font_size: NonZeroPositiveF32::new(text.font_size).unwrap(),
                            font: Font {
                                families: font_families,
                                style: if text.italic {
                                    FontStyle::Italic
                                } else {
                                    FontStyle::Normal
                                },
                                weight: 12,
                                stretch: usvg::FontStretch::Normal,
                            },
                            fill: Some(usvg::Fill {
                                paint: usvg::Paint::Color(usvg::Color {
                                    red: text.color.0,
                                    green: text.color.1,
                                    blue: text.color.2,
                                }),
                                ..Default::default()
                            }),
                            stroke: None,
                            small_caps: false,
                            word_spacing: 0.0,
                            letter_spacing: 0.0,
                            apply_kerning: true,
                            decoration: usvg::TextDecoration {
                                underline: None,
                                overline: None,
                                line_through: None,
                            },
                            baseline_shift: vec![],
                            paint_order: usvg::PaintOrder::FillAndStroke,
                            visibility: usvg::Visibility::Visible,
                            dominant_baseline: usvg::DominantBaseline::Auto,
                            alignment_baseline: usvg::AlignmentBaseline::Auto,
                            length_adjust: usvg::LengthAdjust::Spacing,
                            text_length: None,
                        }],
                        text_flow: usvg::TextFlow::Linear,
                    }],
                }))
            }
            Item::Image(image) => usvg::Node::Image(Box::new(usvg::Image {
                id: String::new(),
                abs_transform: Transform::identity(), // Set on postprocessing, not here
                bounding_box: None,
                visibility: usvg::Visibility::Visible,
                view_box: usvg::ViewBox {
                    rect: usvg::NonZeroRect::from_xywh(
                        0.,
                        0.,
                        image.viewport_width,
                        image.viewport_height,
                    )
                    .unwrap(),
                    aspect: usvg::AspectRatio::default(),
                },
                rendering_mode: usvg::ImageRendering::OptimizeSpeed,
                kind: match &image.data {
                    ItemImageData::Png(data) => usvg::ImageKind::PNG(data.clone()),
                    ItemImageData::Jpeg(data) => usvg::ImageKind::JPEG(data.clone()),
                    ItemImageData::Gif(data) => usvg::ImageKind::GIF(data.clone()),
                    ItemImageData::Svg(data) => usvg::ImageKind::SVG((**data).clone()),
                },
            })),
        }
    }
}

pub fn render(stage: &Stage, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let tree = stage.to_usvg_tree();

    let tree_size = tree.size.to_int_size();

    let width_scale = canvas_width as f32 / tree_size.width() as f32;
    let height_scale = canvas_width as f32 / tree_size.height() as f32;

    let min_scale = width_scale.min(height_scale);
    log::log(&format!("Width scale: {}", width_scale));
    log::log(&format!("Height scale: {}", height_scale));

    let transform = Transform::from_scale(min_scale, min_scale);

    let mut pixmap = tiny_skia::Pixmap::new(canvas_width, canvas_height).unwrap();
    log::log("Made pixmap");

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    log::log(&format!("Pixmap size: {:?}", tree_size));

    let array: Clamped<&[u8]> = Clamped(pixmap.data());
    log::log(format!("Sizes: {} {}", tree_size.width(), tree_size.height()).as_str());

    let image_data =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(array, canvas_width, canvas_height)?;

    log::log(&format!("Image data: {:?}", image_data));
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    log::log("Put image data");

    Ok(())
}

pub fn render_string(stage: &Stage) -> Result<String, JsValue> {
    let tree = stage.to_usvg_tree();
    let s = tree.to_string(&XmlOptions::default());
    log::log(&format!("Tree: {}", s));
    Ok(s)
}
