use bevy::prelude::*;
use bevy::render::camera::{Camera, VisibleEntities};

#[derive(Bundle, Debug)]
pub struct HiltPerspectiveCameraBundle {
    pub camera: Camera,
    pub perspective_projection: bevy::render::camera::PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HiltPerspectiveCameraBundle {
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

#[derive(Bundle, Debug)]
pub struct HiltOrthographicCameraBundle {
    pub camera: Camera,
    pub orthographic_projection: bevy::render::camera::OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HiltOrthographicCameraBundle {
    fn default() -> Self {
        let OrthographicCameraBundle {
            camera,
            orthographic_projection,
            visible_entities,
            transform,
            global_transform,
        } = OrthographicCameraBundle::with_name(&crate::render::CAMERA_HILT.to_string());
        Self {
            camera,
            orthographic_projection,
            visible_entities,
            transform,
            global_transform,
        }
    }
}
