use crate::{
    models::templates::{
        prop::Prop,
        prop_item::{PropItem, PropItemImage},
    },
    svg, PyramusError,
};
use glam::Affine2;
use resvg::usvg::{self, Transform};

use super::{base_item::Base, item::StageItem, staging::Staging};

/// Trait for converting items to usvg nodes
pub trait ToUsvgNode {
    /// Convert the item to a usvg node
    fn to_usvg_node(&self, base: &Base) -> crate::Result<usvg::Node>;
    /// Convert the item to the outline of a usvg node (for example, for displaying selection)
    fn to_outline_svg_node(&self, base: &Base) -> crate::Result<usvg::Node>;
}

impl ToUsvgNode for StageItem {
    fn to_usvg_node(&self, outer_base_item: &Base) -> crate::Result<usvg::Node> {
        match self {
            StageItem::PropItem(item) => item.to_usvg_node(outer_base_item),
            StageItem::Prop(prop) => prop.to_usvg_node(outer_base_item),
        }
    }

    fn to_outline_svg_node(&self, base: &Base) -> crate::Result<usvg::Node> {
        match self {
            StageItem::PropItem(item) => item.to_outline_svg_node(base),
            StageItem::Prop(prop) => prop.to_outline_svg_node(base),
        }
    }
}

impl ToUsvgNode for Prop {
    // TODO: This may be able to be abstracted in the way that to_outline_svg_node is done
    fn to_usvg_node(&self, outer_base_item: &Base) -> crate::Result<usvg::Node> {
        // TODO: Transforming is not done yet- doesnt inheret from parents, and also scaling seems to move the object
        let transform = to_transform(self.get_relative_transform().to_glam_affine());

        // Recursively add children to the root node
        // TODO: A slotmap may improve this, as we no longer need to hold a lock on the root node
        let root_id = self.template.root;
        let root: &StageItem = self.template.items.get(&root_id).ok_or_else(|| {
            PyramusError::OtherError("Root item not found in template".to_string())
        })?;

        // Create object- recursive, creates propitems within props
        // We use this as our base item for the internal prop item recursion
        // TODO: no clone, use reference, traits
        let base = Base::new(self.clone().into());
        let mut children = vec![root.to_usvg_node(&base)?];

        // Children in scene other props, if any
        for child in self.get_children() {
            // simplify
            let child = outer_base_item.get_items().get(child).ok_or_else(|| {
                PyramusError::OtherError(format!("Child prop item not found in prop: {:?}", child))
                // TODO: Not found could be abstracted
            })?;
            children.push(child.to_usvg_node(outer_base_item)?);
        }

        Ok(usvg::Node::Group(Box::new(usvg::Group {
            transform,
            children,
            ..Default::default()
        })))
    }

    fn to_outline_svg_node(&self, base: &Base) -> crate::Result<usvg::Node> {
        let staged_item = base.get_item(self.id).unwrap(); // todo: shouldnt get a second one
        let bounds = staged_item.get_local_bounds();
        create_outline_svg(staged_item, base, bounds)
    }

    // Get bounds of node
}

impl ToUsvgNode for PropItem {
    fn to_usvg_node(&self, outer_base_item: &Base) -> crate::Result<usvg::Node> {
        crate::log!(
            "Creating prop item node, with children: {:?}",
            self.get_children()
        );
        // TODO: Transforming is not done yet- doesnt inheret from parents, and also scaling seems to move the object
        let transform = to_transform(self.get_relative_transform().to_glam_affine());

        // All nodes are contained in a group node, so we can apply the transform to the group node, and then apply the transform to the children nodes
        // TODO: Is this needed?
        let mut children = vec![self.item.to_usvg_node()?];
        for child in self.get_children() {
            // simplify
            let child = outer_base_item.get_items().get(child).ok_or_else(|| {
                PyramusError::OtherError("Child item not found in stage".to_string())
            })?;
            children.push(child.to_usvg_node(outer_base_item)?);
        }

        Ok(usvg::Node::Group(Box::new(usvg::Group {
            transform,
            children,
            ..Default::default()
        })))
    }

    fn to_outline_svg_node(&self, base: &Base) -> crate::Result<usvg::Node> {
        let staged_item = base.get_item(self.id).unwrap(); // todo: shouldnt get a second one
        let bounds = staged_item.get_local_bounds();
        create_outline_svg(staged_item, base, bounds)
    }
}

fn to_transform(transform: Affine2) -> usvg::Transform {
    let cols = transform.to_cols_array();
    usvg::Transform::from_row(cols[0], cols[1], cols[2], cols[3], cols[4], cols[5])
}

fn create_outline_svg(
    staged_item: &StageItem,
    base: &Base,
    (x0, y0, x1, y1): (f32, f32, f32, f32),
) -> crate::Result<usvg::Node> {
    let outline_size = 20.0;
    // Get bounds of node
    // TODO: NEed consistency between x1x2 and xywh formats

    // TODO: This is terrible, and should be done via traits
    let transform = to_transform(staged_item.get_screen_transform(base));

    let x0 = x0 - outline_size;
    let y0 = y0 - outline_size;
    let x1 = x1 + outline_size;
    let y1 = y1 + outline_size;

    crate::log!(
        "Outline for : {x0} {y0}, {x1} {y1}, width: {w}, height: {h}",
        w = x1 - x0,
        h = y1 - y0
    );
    let image = usvg::Node::Image(Box::new(usvg::Image {
        id: String::new(),
        abs_transform: Transform::identity(), // Set on postprocessing, not here
        bounding_box: None,
        visibility: usvg::Visibility::Visible,
        view_box: usvg::ViewBox {
            rect: usvg::NonZeroRect::from_ltrb(x0, y0, x1, y1).ok_or_else(|| {
                crate::log!("Invalid size to_outline_svg_node: {x0}, {y0}, {x1}, {y1}");
                PyramusError::InvalidSize(x1 - x0, y1 - y0)
            })?,
            aspect: usvg::AspectRatio::default(),
        },
        rendering_mode: usvg::ImageRendering::OptimizeSpeed,
        kind: PropItemImage::from_svg_string(&svg::build_svg_rect(
            (x1 - x0) as u32,
            (y1 - y0) as u32,
            "blue",
            Some(outline_size as u32),
            0.5,
        ))?
        .data
        .into(),
    }));

    Ok(usvg::Node::Group(Box::new(usvg::Group {
        transform,
        children: vec![image],
        ..Default::default()
    })))
}
