use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::entities::*;
use crate::render::WireframeMaterial;

pub fn spawn_debug_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
                    let mut co_transform = Transform::from_translation(Vec3::from(co_pos.translation) - Vec3::from(rb_pos.position.translation));
                    co_transform.rotation = Quat::from(co_pos.rotation);
                    co_transform
                } else {
                    Default::default()
                }
            } else {
                let mut transform = Transform::from_translation(co_pos.translation.into());
                transform.rotation = co_pos.rotation.into();
                transform
            }
        };
        commands.entity(entity)
            .insert(HiltDebugColliderLoaded)
            .insert(Visible { is_visible: true, is_transparent: true })
            .with_children(|parent| {
                if let Some(mesh) = collider_to_mesh(shape) {
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

fn collider_to_mesh(shape: &ColliderShape) -> Option<Mesh> {
    match shape.shape_type() {
        ShapeType::Cuboid => {
            let cuboid = shape.as_cuboid().unwrap();
            Some(crate::mesh::wire_cube(cuboid))
        },
        ShapeType::Ball => {
            let ball = shape.as_ball().unwrap();
            Some(crate::mesh::wire_sphere(ball.radius))
        },
        ShapeType::TriMesh => {
            let trimesh = shape.as_trimesh().unwrap();
            Some(crate::mesh::wire_trimesh(trimesh))
        },
        ShapeType::Capsule => {
            let capsule = shape.as_capsule().unwrap();
            Some(crate::mesh::wire_capsule(capsule))
        },
        ty => {
            warn!("Unable to render collider shape type: {:?}", ty);
            None
        }
    }
}
