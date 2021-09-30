use bevy::prelude::*;

mod collider;
pub use self::collider::HiltDebugColliderWireframeBundle;

mod path;
pub use self::path::HiltDebugPathWireframeBundle;

mod position;
pub use self::position::{HiltDebugPositionBundle, HiltDebugPositionSize};

mod camera;
pub use self::camera::{
    HiltOrthographicCameraBundle,
    HiltPerspectiveCameraBundle
};

pub struct HiltDebugColliderLoaded;
pub struct HiltDebugPositionLoaded;
pub struct HiltDebugPathLoaded(pub Entity);

pub struct HiltDebugRenderCollider;
pub struct HiltDebugRenderPosition;
pub struct HiltDebugRenderPath;

#[derive(Debug)]
pub struct HiltDebugCollider {
    pub color: Color
}

impl HiltDebugCollider {

    pub fn new(color: Color) -> HiltDebugCollider {
        HiltDebugCollider { color }
    }
}

#[derive(Debug)]
pub struct HiltDebugPath {
    pub color: Color,
    pub length: usize,
    pub dashed: bool
}

impl Default for HiltDebugPath {
    fn default() -> HiltDebugPath {
        HiltDebugPath {
            color: Color::RED,
            length: 1000,
            dashed: false
        }
    }
}

#[derive(Debug)]
pub struct HiltDebugPosition {
    pub size: f32,
    pub x: Color,
    pub y: Color,
    pub z: Color
}

impl Default for HiltDebugPosition {
    fn default() -> HiltDebugPosition {
        HiltDebugPosition {
            #[cfg(feature = "3d")]
            size: 0.2,
            #[cfg(feature = "2d")]
            size: 10.0,
            x: Color::RED,
            y: Color::BLUE,
            z: Color::GREEN
        }
    }
}
