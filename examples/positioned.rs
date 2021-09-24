use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hilt::prelude::*;

mod utils;

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn()
        .insert(Name::new("Objects"))
        .insert(Transform::from_xyz(0.0, 5.0, 0.0))
        .insert(GlobalTransform::identity())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.5, 0.25, 0.5, 0.5).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: Vec3::new(0.0, 5.0, 0.0).into(),
            ..Default::default()
        })
        .insert(HiltDebugPosition::default())
        .insert(HiltDebugPath {
            dashed: true,
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent.spawn_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(0.25, 0.25, 0.25),
                position: Vec3::new(0.0, -1.5, 0.0).into(),
                material: ColliderMaterial {
                    restitution: 1.25,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .insert(Transform::from_xyz(0.0, 1.5, 0.0))
            .insert(GlobalTransform::identity())
            .insert(HiltDebugCollider {
                color: Color::ORANGE
            })
            .insert(HiltDebugPosition::default());
        });
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(HiltDebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(spawn_scene.system().after("environment"))
        .run();
}
