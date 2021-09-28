use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;

#[cfg(feature = "3d")]
pub fn wire_cube(cuboid: &Cuboid) -> Mesh {
    let x = cuboid.half_extents.x;
    let y = cuboid.half_extents.y;
    let z = cuboid.half_extents.z;
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Front
            [x, y, z],
            [x, -y, z],
            [-x, -y, z],
            [-x, y, z],
            // Back
            [x, y, -z],
            [x, -y, -z],
            [-x, -y, -z],
            [-x, y, -z],
        ],
    );
    mesh.set_indices(Some(bevy::render::mesh::Indices::U16(vec![
        0, 1, 1, 2, 2, 3, 3, 0, // Front
        4, 5, 5, 6, 6, 7, 7, 4, // Back
        0, 4, 1, 5, 2, 6, 3, 7, // Bridge
    ])));
    mesh
}

#[cfg(feature = "2d")]
pub fn wire_cube(cuboid: &Cuboid) -> Mesh {
    let x = cuboid.half_extents.x;
    let y = cuboid.half_extents.y;
//    let z = cuboid.half_extents.z;
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Front
            [x, y],
            [x, -y],
            [-x, -y],
            [-x, y],
            // Back
//            [x, y, -z],
//            [x, -y, -z],
//            [-x, -y, -z],
//            [-x, y, -z],
        ],
    );
    mesh.set_indices(Some(bevy::render::mesh::Indices::U16(vec![
        0, 1, 1, 2, 2, 3, 3, 0, // Front
//        4, 5, 5, 6, 6, 7, 7, 4, // Back
//        0, 4, 1, 5, 2, 6, 3, 7, // Bridge
    ])));
    mesh
}
