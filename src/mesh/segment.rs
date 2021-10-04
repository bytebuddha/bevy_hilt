use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::VertexAttributeValues;
#[cfg(feature = "3d")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "2d")]
use bevy_rapier2d::prelude::*;

pub fn wire_segment(segment: &Segment) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            vec![
                [segment.a.x, segment.a.y],
                [segment.b.x, segment.b.y]
            ],
        ),
    );
    mesh
}
