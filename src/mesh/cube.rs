use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use bevy_rapier3d::prelude::Cuboid;

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
