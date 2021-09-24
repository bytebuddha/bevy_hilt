mod wireframe_material;
pub use self::wireframe_material::WireframeMaterial;

mod position_material;
pub use self::position_material::PositionWireframeMaterial;

mod setup;
pub use self::setup::setup_material;

mod pass;
pub use self::pass::setup_hilt_pass;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::PipelineDescriptor;

#[derive(Debug, Clone, Default, bevy::render::renderer::RenderResources)]
pub struct HiltPass;

pub const CAMERA_HILT: &str = "camera_hilt";
pub const HILT_PASS: &str = "hilt_pass";
pub const HILT_MESH: &str = "hilt_mesh";

pub const COLLIDER_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x936896ad9d35720c_u64);

pub const POSITION_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x0807a09e1c05e8b0_u64);



pub struct ColliderWireframePipelineDescriptor(bevy::asset::Handle<bevy::render::pipeline::PipelineDescriptor>);
pub struct PositionPipelineDescriptor(bevy::asset::Handle<bevy::render::pipeline::PipelineDescriptor>);
