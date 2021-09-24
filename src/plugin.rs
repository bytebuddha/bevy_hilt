use bevy::prelude::*;
use bevy::render::shader::asset_shader_defs_system;

pub struct HiltDebugPlugin;

impl Plugin for HiltDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<crate::render::WireframeMaterial>()
            .add_asset::<crate::render::PositionWireframeMaterial>()
            .add_event::<crate::HiltToggleVisibility>()
            .add_event::<crate::HiltToggleRenderPass>()
            .add_startup_system(crate::render::setup_material.system().label("material_setup"))
            .add_startup_system(crate::render::setup_hilt_pass.system().after("material_setup"))
            .add_system(crate::systems::spawn_debug_colliders.system())
            .add_system(crate::systems::spawn_debug_positions.system())
            .add_system(crate::systems::spawn_debug_paths.system())
            .add_system(crate::systems::update_path_mesh.system())
            .add_system(crate::systems::toggle_visibility.system())
            .add_system(crate::systems::toggle_render_pass.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                asset_shader_defs_system::<crate::render::WireframeMaterial>.system(),
            );

    }
}
