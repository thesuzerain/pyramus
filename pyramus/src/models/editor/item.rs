use crate::models::blueprint::prop::PropItem;

use super::stage::Stage;
use glam::Vec2;

// TODO: I don't like this being here- this was StagedItem
// This should be made into a trait that both Prop and PropItem implement
impl PropItem {
    pub fn contains_point(&self, x: f32, y: f32, stage: &Stage) -> bool {
        // Get transform of current item
        let transform = self.get_screen_transform(stage);

        // Get the click in local space and check if it's within the bounds of the item
        let click = transform.inverse().transform_point2(glam::Vec2::new(x, y));
        let (x0, y0, x1, y1) = self.item.get_local_bounds();
        click.x >= x0 && click.x <= x1 && click.y >= y0 && click.y <= y1
    }

    pub fn get_screen_transform(&self, stage: &Stage) -> glam::Affine2 {
        // TODO: If we add 3d, this needs a projection matrix/camera and world space as an intermediate step
        let transform = self.transform.to_glam_affine();
        if let Some(parent_id) = self.parent {
            let parent_item = stage
                .base
                .items
                .get(&parent_id)
                .expect("Parent item not found");
            transform * parent_item.get_screen_transform(stage)
        } else {
            transform
        }
    }

    // x0, y0, x1, y1
    pub fn get_bounds(&self, stage: &Stage) -> (f32, f32, f32, f32) {
        let (x0, y0, x1, y1) = self.item.get_local_bounds();

        let transform = self.get_screen_transform(stage);
        crate::log!(
            "Transform for {name}: {transform}, onto {x0} {y0}, {x1} {y1}",
            name = self.name
        );
        let Vec2 { x: x0, y: y0 } = transform.transform_point2(glam::Vec2::new(x0, y0));
        let Vec2 { x: x1, y: y1 } = transform.transform_point2(glam::Vec2::new(x1, y1));

        (
            f32::min(x0, x1),
            f32::min(y0, y1),
            f32::max(x0, x1),
            f32::max(y0, y1),
        )
    }
}
