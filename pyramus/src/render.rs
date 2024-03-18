use crate::{
    models::{
        editor::{item::StageItem, stage::Stage},
        templates::prop_item::{PropItem, PropItemType},
    },
    PyramusError,
};
use svgtypes::parse_font_families;

use resvg::usvg::{self, Font, FontStyle, TextSpan, Transform, XmlOptions};
use usvg::fontdb;

impl Stage {
    pub fn to_usvg_tree(&self) -> crate::Result<usvg::Tree> {
        let width = self.base.get_size().0 as f32;
        let height = self.base.get_size().1 as f32;
        let mut tree = usvg::Tree {
            size: usvg::Size::from_wh(width, height)
                .ok_or_else(|| PyramusError::InvalidSize(width, height))?,
            view_box: usvg::ViewBox {
                // TODO: Look here for viewbox- will need to revisit this section as we do resizing
                rect: usvg::NonZeroRect::from_xywh(0., 0., width, height)
                    .ok_or_else(|| PyramusError::InvalidSize(width, height))?,
                aspect: usvg::AspectRatio::default(),
            },
            root: usvg::Group::default(),
        };

        // Recursively add children to the root node
        // TODO: A slotmap may improve this, as we no longer need to hold a lock on the root node
        let root: &StageItem = self
            .base
            .get_item(self.base.get_root())
            .ok_or_else(|| PyramusError::OtherError("Root item not found in stage".to_string()))?;

        {
            tree.root.children.push(root.to_usvg_node(self)?);
        }

        // Add outlines overtop of the nodes
        for item in self.get_selections() {
            tree.root.children.push(item.to_outline_svg_node(self)?);
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

        Ok(tree)
    }
}

// TODO: Trait renderable
impl PropItem {
    // From Graphite
    // TODO: you have multiple uses of this
}

impl PropItemType {
    pub fn to_usvg_node(&self) -> crate::Result<usvg::Node> {
        match &self {
            PropItemType::Text(text) => {
                // TODO: There doesn't seem to be a way in resvg to create a text node directly/simply.
                // An alternative would be simply parsing a string- but that's hacky, and it might reload fonts.
                // TODO: Check if it reloads fonts, and/or find a way to do this more simply.

                // TODO: pass fontdb or a config containing it through, or get from a state
                let mut fontdb = fontdb::Database::new();
                fontdb.load_system_fonts();

                // todo: shouldn't hardcode this
                let font_families = parse_font_families(&text.font_family)?;

                let num_chars = text.text.chars().count();
                let node = usvg::Node::Text(Box::new(usvg::Text {
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
                            font_size: text.font_size,
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
                }));
                Ok(node)
            }
            PropItemType::Image(image) => Ok(usvg::Node::Image(Box::new(usvg::Image {
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
                    .ok_or_else(|| {
                        PyramusError::InvalidSize(image.viewport_width, image.viewport_height)
                    })?,
                    aspect: usvg::AspectRatio::default(),
                },
                rendering_mode: usvg::ImageRendering::OptimizeSpeed,
                kind: image.data.clone().into(),
            }))),
        }
    }
}

pub fn render_string(stage: &Stage) -> crate::Result<String> {
    let tree = stage.to_usvg_tree()?;
    let s = tree.to_string(&XmlOptions::default());
    Ok(s)
}
