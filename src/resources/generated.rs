use bevy::{prelude::*, render::mesh, render::render_resource::PrimitiveTopology};

pub struct GeneratedMeshes {
    pub tile_indicator: Handle<Mesh>,
    pub tile_color: Handle<ColorMaterial>,
    pub tile_color_alt: Handle<ColorMaterial>,
}
impl GeneratedMeshes {
    pub fn load(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> GeneratedMeshes {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        // Positions of the vertices
        // See https://bevy-cheatbook.github.io/features/coords.html
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-32., -32., 0.],
                [0., -16., 0.],
                [32., -32., 0.],
                [-32., -32., 0.],
                [0., -48., 0.],
                [32., -32., 0.],
            ],
        );

        // In this example, normals and UVs don't matter,
        // so we just use the same value for all of them
        mesh.compute_flat_normals();
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

        // A triangle using vertices 0, 2, and 1.
        // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
        mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1, 3, 5, 4])));

        // meshes
        //     .add(shape::Quad::new(Vec2::new(50., 100.)).into())
        //     .into()

        GeneratedMeshes {
            tile_indicator: meshes.add(mesh),
            tile_color: materials.add(ColorMaterial::from(Color::LIME_GREEN.with_a(0.5))),
            tile_color_alt: materials.add(ColorMaterial::from(Color::BLUE.with_a(0.5))),
        }
    }
}
