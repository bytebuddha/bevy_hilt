mod mesh;

pub mod render;
pub mod entities;
pub mod systems;

mod plugin;
pub use self::plugin::HiltDebugPlugin;

pub mod prelude {
    pub use crate::entities::*;
    pub use crate::{HiltDebugPlugin, HiltToggleVisibility, HiltToggleRenderPass, HiltEntities};
}

#[derive(Debug)]
pub enum HiltEntities {
    All,
    Colliders,
    Positions,
    Path
}

#[derive(Debug)]
pub struct HiltToggleVisibility(pub HiltEntities);

#[derive(Debug)]
pub struct HiltToggleRenderPass(pub HiltEntities);
