use bevy::prelude::*;
use crate::entities::*;
use crate::render::PositionWireframeMaterial;

pub fn spawn_debug_positions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
                    mesh: meshes.add(crate::mesh::position_mesh(debug)),
                    material: materials.add(PositionWireframeMaterial::default()),
                    ..Default::default()
                });
            });
    }
}
