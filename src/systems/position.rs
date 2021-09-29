use bevy::prelude::*;
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;
use crate::entities::*;
use crate::render::PositionWireframeMaterial;

pub fn spawn_debug_positions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<RapierConfiguration>,
    mut materials: ResMut<Assets<PositionWireframeMaterial>>,
    query: Query<(Entity, &HiltDebugPosition), Without<HiltDebugPositionLoaded>>
) {
    for (entity, debug) in query.iter() {
        commands.entity(entity)
            .insert(HiltDebugPositionLoaded)
            .insert(Visible { is_visible: true, is_transparent: true })
            .with_children(|parent| {
                parent.spawn()
                .insert(Name::new("Hilt Position"))
                .insert(HiltDebugRenderPosition)
                .insert_bundle(HiltDebugPositionBundle {
                    size: HiltDebugPositionSize(debug.size),
                    mesh: meshes.add(crate::mesh::position_mesh(debug, &config)),
                    material: materials.add(PositionWireframeMaterial::default()),
                    ..Default::default()
                });
            });
    }
}
