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
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .with_children(|parent| {
            for x_coord in 1..4 {
                for y_coord in 1..4 {
                    let pos_x = (x_coord as f32 * 2.0) - 4.0;
                    let pos_y = (x_coord * y_coord) as f32 * 0.8;
                    let pos_z = (y_coord as f32 * 2.0) - 4.0;
                    let (shape, display) = generate_shape(x_coord, y_coord);
                    parent.spawn_bundle(PbrBundle {
                            mesh: meshes.add(display),
                            material: materials.add(Color::BLACK.into()),
                            ..Default::default()
                        })
                        .insert_bundle(RigidBodyBundle {
                            body_type: RigidBodyType::Dynamic,
                            position: Vec3::new(pos_x, pos_y, pos_z).into(),
                            ..Default::default()
                        })
                        .insert(RigidBodyPositionSync::Discrete)
                        .insert(HiltDebugCollider { color: coord_color(x_coord) })
                        .insert(HiltDebugPosition::default())
                        .insert(HiltDebugPath::default())
                        .insert_bundle(ColliderBundle {
                            shape,
                            mass_properties: ColliderMassProps::Density(50.0),
                            material: ColliderMaterial {
                                restitution: 0.8 + (x_coord * y_coord) as f32 * 0.1,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Name::new(format!("Object({}, {})", x_coord, y_coord)));
                }
            }
        });
}

fn generate_shape(col: usize, row: usize) -> (SharedShape, Mesh) {
    let vec = col as f32 * 0.2;
    match row {
        1 => (
            ColliderShape::ball(0.25 + vec),
            Mesh::from(bevy::prelude::shape::Icosphere { radius: 0.25 + vec, subdivisions: 1 })
        ),
        2 => (
            ColliderShape::cuboid(0.25 + vec, 0.25 + vec, 0.25 + vec),
            Mesh::from(bevy::prelude::shape::Box::new(0.50 + vec*2., 0.50 + vec*2., 0.50 + vec*2.))
        ),
        3 => (
            ColliderShape::capsule(
                Vec3::new(0.0, 0.0, 0.0).into(),
                Vec3::new(0.0, vec + 1.0, 0.0).into(),
                0.25 + vec
            ),
            Mesh::from(bevy::prelude::shape::Capsule {
                    radius: 0.125 + vec/2.0,
                    rings: 0,
                    depth: vec - 0.125,
                    latitudes: 6,
                    longitudes: 14,
                    uv_profile: bevy::prelude::shape::CapsuleUvProfile::Aspect
            })
        ),
        _ => unreachable!()
    }
}

pub fn coord_color(x: usize) -> Color {
    match x {
        1 => Color::MAROON,
        2 => Color::TEAL,
        3 => Color::FUCHSIA,
        _ => Color::ORANGE
    }
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
