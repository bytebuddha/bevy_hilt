use bevy::prelude::*;
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;
use bevy_hilt::prelude::*;

mod utils;

#[cfg(feature = "3d")]
const DIMENSION_SCALE: f32 = 1.0;
#[cfg(feature = "2d")]
const DIMENSION_SCALE: f32 = 100.0;

fn spawn_scene(mut commands: Commands) {
    commands.spawn()
        .insert(Name::new("Objects"))
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .with_children(|parent| {
            for x_coord in 1..4 {
                for y_coord in 1..4 {
                    let pos_x = (x_coord as f32 * 2.0 * DIMENSION_SCALE) - 4.0 * DIMENSION_SCALE + y_coord as f32 / 2.0 * DIMENSION_SCALE;
                    let pos_y = (x_coord * y_coord) as f32 * 0.8 * DIMENSION_SCALE;
                    #[cfg(feature = "3d")]
                    let pos_z = (y_coord as f32 * 2.0 * DIMENSION_SCALE) - 4.0 * DIMENSION_SCALE;
                    let shape = generate_shape(x_coord, y_coord);
                    parent.spawn_bundle(RigidBodyBundle {
                            body_type: RigidBodyType::Dynamic,
                            #[cfg(feature = "3d")]
                            position: Vec3::new(pos_x, pos_y, pos_z).into(),
                            #[cfg(feature = "2d")]
                            position: Vec2::new(pos_x, pos_y).into(),
                            ..Default::default()
                        })
                        .insert(RigidBodyPositionSync::Discrete)
                        .insert(HiltDebugCollider { color: coord_color(x_coord) })
                        .insert(HiltDebugPosition::default())
                        .insert(HiltDebugPath::default())
                        .insert_bundle(ColliderBundle {
                            shape,
                            mass_properties: ColliderMassProps::Density(50.0 * x_coord as f32),
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

fn generate_shape(col: usize, row: usize) -> SharedShape {
    let vec = col as f32 * 0.2 * DIMENSION_SCALE;
    match row {
        1 => ColliderShape::ball(0.5 / DIMENSION_SCALE + vec),
        #[cfg(feature = "3d")]
        2 => ColliderShape::cuboid(0.25 * DIMENSION_SCALE + vec, 0.25 * DIMENSION_SCALE + vec, 0.25 * DIMENSION_SCALE + vec),
        #[cfg(feature = "2d")]
        2 => ColliderShape::cuboid(0.25 * DIMENSION_SCALE + vec, 0.25 * DIMENSION_SCALE + vec),
        3 => ColliderShape::capsule(
                #[cfg(feature = "3d")]
                Vec3::new(0.0, 0.0, 0.0).into(),
                #[cfg(feature = "2d")]
                Vec2::new(0.0, 0.0).into(),
                #[cfg(feature = "3d")]
                Vec3::new(0.0, vec + 1.0 * DIMENSION_SCALE, 0.0).into(),
                #[cfg(feature = "2d")]
                Vec2::new(0.0, vec + 1.0 * DIMENSION_SCALE).into(),
                0.25 + vec
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
