use bevy::prelude::*;
use bevy::render::camera::{PerspectiveProjection, Camera, VisibleEntities};

#[derive(Bundle, Debug)]
pub struct HiltCameraBundle {
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HiltCameraBundle {
    fn default() -> Self {
        let PerspectiveCameraBundle {
            camera,
            perspective_projection,
            visible_entities,
            transform,
            global_transform,
        } = PerspectiveCameraBundle::with_name(&crate::render::CAMERA_HILT.to_string());
        Self {
            camera,
            perspective_projection,
            visible_entities,
            transform,
            global_transform,
        }
    }
}
