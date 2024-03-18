use crate::{
    models::templates::{
        ids::ItemId,
        prop::Prop,
        prop_item::{PropItem, PropItemImage},
        transform::RelativeTransform,
    },
    svg, PyramusError,
};

use super::stage::Stage;
use glam::{Affine2, Vec2};
use resvg::usvg::{self, Transform};

#[derive(Debug)]
pub enum StageItem {
    PropItem(PropItem),
    Prop(Prop),
}

impl StageItem {
    // TODO: should we keep these as the trait?

    pub fn get_id(&self) -> ItemId {
        match self {
            StageItem::PropItem(item) => item.id,
            StageItem::Prop(prop) => prop.id,
        }
    }

    pub fn rename(&mut self, name: String) {
        match self {
            StageItem::PropItem(item) => item.name = name,
            StageItem::Prop(prop) => prop.name = name,
        }
    }

    pub fn get_parent(&self) -> Option<ItemId> {
        match self {
            StageItem::PropItem(item) => item.parent,
            StageItem::Prop(prop) => prop.parent,
        }
    }

    pub fn set_parent(&mut self, parent: Option<ItemId>) {
        match self {
            StageItem::PropItem(item) => item.parent = parent,
            StageItem::Prop(prop) => prop.parent = parent,
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let (x0, y0, x1, x2) = match self {
            StageItem::PropItem(item) => item.get_local_bounds(),
            StageItem::Prop(prop) => prop.get_local_bounds(),
        };
        ((x1 - x0) as u32, (x2 - y0) as u32)
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<ItemId> {
        match self {
            StageItem::PropItem(item) => &mut item.children,
            StageItem::Prop(prop) => &mut prop.children,
        }
    }

    pub fn get_children(&self) -> &Vec<ItemId> {
        match self {
            StageItem::PropItem(item) => &item.children,
            StageItem::Prop(prop) => &prop.children,
        }
    }

    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        match self {
            StageItem::PropItem(item) => item.get_local_bounds(),
            StageItem::Prop(prop) => prop.get_local_bounds(),
        }
    }

    pub fn get_relative_transform(&self) -> &RelativeTransform {
        match self {
            StageItem::PropItem(item) => &item.transform,
            StageItem::Prop(prop) => &prop.transform,
        }
    }

    pub fn get_relative_transform_mut(&mut self) -> &mut RelativeTransform {
        match self {
            StageItem::PropItem(item) => &mut item.transform,
            StageItem::Prop(prop) => &mut prop.transform,
        }
    }

    /// Check if a point in screen space is within the bounds of the item
    pub fn contains_point(&self, x: f32, y: f32, stage: &Stage) -> bool {
        // Get transform of current item
        let transform = self.get_screen_transform(stage);

        // Get the click in local space and check if it's within the bounds of the item
        let click = transform.inverse().transform_point2(glam::Vec2::new(x, y));
        let (x0, y0, x1, y1) = self.get_local_bounds();
        click.x >= x0 && click.x <= x1 && click.y >= y0 && click.y <= y1
    }

    /// Get the bounds of the item in screen space
    /// x0, y0, x1, y1
    pub fn get_bounds(&self, stage: &Stage) -> (f32, f32, f32, f32) {
        let (x0, y0, x1, y1) = self.get_local_bounds();

        let transform = self.get_screen_transform(stage);
        let Vec2 { x: x0, y: y0 } = transform.transform_point2(glam::Vec2::new(x0, y0));
        let Vec2 { x: x1, y: y1 } = transform.transform_point2(glam::Vec2::new(x1, y1));

        (
            f32::min(x0, x1),
            f32::min(y0, y1),
            f32::max(x0, x1),
            f32::max(y0, y1),
        )
    }

    /// Get the transform of the item in screen space
    pub fn get_screen_transform(&self, stage: &Stage) -> glam::Affine2 {
        // TODO: If we add 3d, this needs a projection matrix/camera and world space as an intermediate step
        let transform = self.get_relative_transform().to_glam_affine();
        if let Some(parent_id) = self.get_parent() {
            let parent_item = stage
                .base
                .get_item(parent_id)
                .expect("Parent item not found");
            parent_item.get_screen_transform(stage) * transform
        } else {
            transform
        }
    }

    /// Convert the item to a usvg node
    pub fn to_usvg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        match self {
            StageItem::PropItem(item) => item.to_usvg_node(stage),
            StageItem::Prop(prop) => prop.to_usvg_node(stage),
        }
    }

    /// Convert the item to the outline of a usvg node
    pub fn to_outline_svg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        match self {
            StageItem::PropItem(item) => item.to_outline_svg_node(stage),
            StageItem::Prop(prop) => prop.to_outline_svg_node(stage),
        }
    }
}

impl Prop {
    fn to_usvg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        let mut group = usvg::Group::default();

        let root_id = self.template.root;

        // Recursively add children to the root node
        // TODO: A slotmap may improve this, as we no longer need to hold a lock on the root node
        let root: &StageItem = self.template.items.get(&root_id).ok_or_else(|| {
            PyramusError::OtherError("Root item not found in template".to_string())
        })?;

        {
            group.children.push(root.to_usvg_node(stage)?);
        }

        Ok(usvg::Node::Group(Box::new(group)))
    }

    // TODO: almost identical to to the other one, abstract this
    fn to_outline_svg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        let outline_size = 20.0;

        // Get bounds of node
        // TODO: NEed consistency between x1x2 and xywh formats
        let (x0, y0, x1, y1) = self.get_local_bounds();

        // TODO: This is terrible, and should be done via traits
        let staged_item = stage.base.get_item(self.id).unwrap(); // todo: shouldnt get a second one
        let transform = to_transform(staged_item.get_screen_transform(stage));

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
}

// TODO: I don't like this being here- this was StagedItem
// This should be made into a trait that both Prop and PropItem implement
impl PropItem {
    fn to_usvg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        // TODO: Transforming is not done yet- doesnt inheret from parents, and also scaling seems to move the object
        let transform = to_transform(self.transform.to_glam_affine());

        // All nodes are contained in a group node, so we can apply the transform to the group node, and then apply the transform to the children nodes
        // TODO: Is this needed?
        let mut children = vec![self.item.to_usvg_node()?];
        for child in &self.children {
            // simplify
            let child = stage.base.get_template().items.get(child).ok_or_else(|| {
                PyramusError::OtherError("Child item not found in stage".to_string())
            })?;
            children.push(child.to_usvg_node(stage)?);
        }

        Ok(usvg::Node::Group(Box::new(usvg::Group {
            transform,
            children,
            ..Default::default()
        })))
    }

    fn to_outline_svg_node(&self, stage: &Stage) -> crate::Result<usvg::Node> {
        let outline_size = 20.0;

        // Get bounds of node
        // TODO: NEed consistency between x1x2 and xywh formats
        let (x0, y0, x1, y1) = self.item.get_local_bounds();

        // TODO: This is terrible, and should be done via traits
        let staged_item = stage.base.get_item(self.id).unwrap(); // todo: shouldnt get a second one
        let transform = to_transform(staged_item.get_screen_transform(stage));

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
}

fn to_transform(transform: Affine2) -> usvg::Transform {
    let cols = transform.to_cols_array();
    usvg::Transform::from_row(cols[0], cols[1], cols[2], cols[3], cols[4], cols[5])
}
