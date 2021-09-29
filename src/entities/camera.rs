use bevy::prelude::*;
use bevy::render::camera::{Camera, VisibleEntities};

#[derive(Bundle, Debug)]

pub struct HiltCameraBundle {
    pub camera: Camera,
    #[cfg(feature = "3d")]
    pub perspective_projection: bevy::render::camera::PerspectiveProjection,
    #[cfg(feature = "2d")]
    pub orthographic_projection: bevy::render::camera::OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HiltCameraBundle {
    fn default() -> Self {
        #[cfg(feature = "3d")]
        let PerspectiveCameraBundle {
            camera,
            perspective_projection,
            visible_entities,
            transform,
            global_transform,
        } = PerspectiveCameraBundle::with_name(&crate::render::CAMERA_HILT.to_string());
        #[cfg(feature = "2d")]
        let OrthographicCameraBundle {
            camera,
            orthographic_projection,
            visible_entities,
            transform,
            global_transform,
        } = OrthographicCameraBundle::with_name(&crate::render::CAMERA_HILT.to_string());
        Self {
            camera,
            #[cfg(feature = "3d")]
            perspective_projection,
            #[cfg(feature = "2d")]
            orthographic_projection,
            visible_entities,
            transform,
            global_transform,
        }
    }
}
