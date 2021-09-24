use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hilt::entities::{HiltDebugCollider, HiltCameraBundle};

pub fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
    .insert(Name::new("Plane"))
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(7.5, 0.1, 7.5),
        mass_properties: ColliderMassProps::Density(200.0),
        ..Default::default()
    })
    .insert(HiltDebugCollider { color: Color::GREEN });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    })
    .insert(Name::new("Light 1"));

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-3.0, 8.5, -3.8),
        ..Default::default()
    })
    .insert(Name::new("Light 2"));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-9.0, 5.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(super::Rotates)
    .with_children(|parent| {
        parent.spawn_bundle(HiltCameraBundle {
            ..Default::default()
        });
    });
}
