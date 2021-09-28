use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::{Indices, VertexAttributeValues};
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;

// TODO: Fix this.
#[cfg(feature = "3d")]
pub fn wire_trimesh(trimesh: &TriMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y, vertex.z])
                .collect::<Vec<_>>(),
        ),
    );
    let indicies = trimesh
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[2], triangle[0]].into_iter())
//        .cloned()
        .collect();

    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh
}

#[cfg(feature = "2d")]
pub fn wire_trimesh(trimesh: &TriMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y])
                .collect::<Vec<_>>(),
        ),
    );
    let indicies = trimesh
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[0]].into_iter())
//        .cloned()
        .collect();

    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh
}
