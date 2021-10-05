use bevy::prelude::*;
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;
use crate::entities::*;
use crate::render::WireframeMaterial;

pub fn spawn_debug_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<RapierConfiguration>,
    mut materials: ResMut<Assets<WireframeMaterial>>,
    query: Query<
        (
            Entity, &ColliderShape, &ColliderType, &HiltDebugCollider, &ColliderPosition,
            Option<&RigidBodyPositionSync>, Option<&ColliderPositionSync>,
            Option<&RigidBodyPosition>
        ),
        Without<HiltDebugColliderLoaded>
    >
) {
    for (entity, shape, ty, debug, co_pos, rb_sync, co_sync, rb_pos) in query.iter() {
        let transform = {
            if co_sync.is_some() {
                Transform::identity()
            } else if rb_sync.is_some() {
                if let Some(rb_pos) = rb_pos {
                    rigid_body_transform(rb_pos, co_pos)
                } else {
                    Default::default()
                }
            } else {
                collider_transform(co_pos)
            }
        };
        commands.entity(entity)
            .insert(HiltDebugColliderLoaded)
            .insert(Visible { is_visible: true, is_transparent: true })
            .with_children(|parent| {
                if let Some(mesh) = collider_to_mesh(shape, &config) {
                    parent.spawn()
                        .insert(Name::new("Hilt Collider"))
                        .insert(HiltDebugRenderCollider)
                        .insert_bundle(HiltDebugColliderWireframeBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(crate::render::WireframeMaterial {
                                color: debug.color,
                                dashed: ty == &ColliderType::Sensor
                            }),
                            global_transform: GlobalTransform::from(transform),
                            transform,
                            ..Default::default()
                        });
                }
            });
    }
}

#[cfg(feature = "3d")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    let mut transform = Transform::from_translation(co_pos.translation.into());
    transform.rotation = co_pos.rotation.into();
    transform
}

#[cfg(feature = "2d")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    let transform = Transform::from_xyz(
        co_pos.translation.x,
        co_pos.translation.y,
        1.0
    );
//    transform.rotation = Quat::from_rotation_z(co_pos.rotation.complex().re);
//    transform.rotation = co_pos.rotation.into();
    transform
}

#[cfg(feature = "3d")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let mut co_transform = Transform::from_translation(Vec3::from(co_pos.translation) - Vec3::from(rb_pos.position.translation));
    co_transform.rotation = Quat::from(co_pos.rotation);
    co_transform
}

#[cfg(feature = "2d")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let pos = Vec2::from(co_pos.translation) - Vec2::from(rb_pos.position.translation);
    let co_transform = Transform::from_xyz(pos.x, pos.y, 1.0);
//    co_transform.rotation = Quat::from_rotation_y(co_pos.rotation.complex().re);
    co_transform
}

fn collider_to_mesh(shape: &ColliderShape, config: &RapierConfiguration) -> Option<Mesh> {
    match shape.shape_type() {
        ShapeType::Cuboid => {
            let cuboid = shape.as_cuboid().unwrap();
            Some(crate::mesh::wire_cube(cuboid, config))
        },
        ShapeType::Ball => {
            let ball = shape.as_ball().unwrap();
            Some(crate::mesh::wire_sphere(ball.radius * config.scale))
        },
        ShapeType::TriMesh => {
            let trimesh = shape.as_trimesh().unwrap();
            Some(crate::mesh::wire_trimesh(trimesh))
        },
        ShapeType::Capsule => {
            let capsule = shape.as_capsule().unwrap();
            Some(crate::mesh::wire_capsule(capsule, config))
        },
        ShapeType::Polyline => {
            let polyline = shape.as_polyline().unwrap();
            Some(crate::mesh::wire_polyline(polyline))
        },
        ShapeType::Segment => {
            let segment = shape.as_segment().unwrap();
            Some(crate::mesh::wire_segment(segment))
        },
        #[cfg(feature = "3d")]
        ShapeType::Cylinder => {
            let cylinder = shape.as_cylinder().unwrap();
            Some(crate::mesh::wire_cylinder(cylinder, config))
        },
        ty => {
            warn!("Unable to render collider shape type: {:?}", ty);
            None
        }
    }
}
