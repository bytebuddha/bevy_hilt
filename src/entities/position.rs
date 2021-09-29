use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;

use crate::render::PositionWireframeMaterial;

pub struct HiltDebugPositionSize(pub f32);

impl Default for HiltDebugPositionSize {
    fn default() -> HiltDebugPositionSize {
        #[cfg(feature = "3d")]
        return HiltDebugPositionSize(0.1);
        #[cfg(feature = "2d")]
        HiltDebugPositionSize(10.0)
    }
}

#[derive(Bundle)]
pub struct HiltDebugPositionBundle {
    pub mesh: Handle<Mesh>,
    pub size: HiltDebugPositionSize,
    pub material: Handle<PositionWireframeMaterial>,
    #[cfg(feature = "default_main_pass")]
    pub main_pass: bevy::render::render_graph::base::MainPass,
    #[cfg(not(feature = "default_main_pass"))]
    pub hilt_pass: crate::render::HiltPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HiltDebugPositionBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![
                RenderPipeline::new(
                    crate::render::POSITION_PIPELINE_HANDLE.typed(),
                ),
            ]),
            visible: Visible { is_visible: true, is_transparent: true },
            mesh: Default::default(),
            size: Default::default(),
            material: Default::default(),
            #[cfg(feature = "default_main_pass")]
            main_pass: Default::default(),
            #[cfg(not(feature = "default_main_pass"))]
            hilt_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
