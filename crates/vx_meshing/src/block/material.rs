use bevy::{
    mesh::{MeshVertexAttribute, VertexFormat},
    prelude::*,
    render::render_resource::AsBindGroup,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource)]
pub struct BlockMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub(crate) texture: Handle<Image>,
}

impl BlockMaterial {
    pub const ATTRIBUTE_LAYER: MeshVertexAttribute =
        MeshVertexAttribute::new("Layer", 239_723_840_923_589_702, VertexFormat::Uint32);
}

impl Material for BlockMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/block_array.wgsl".into()
    }

    fn vertex_shader() -> bevy::shader::ShaderRef {
        "shaders/block_array.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::material::descriptor::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::material::specialize::SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Self::ATTRIBUTE_LAYER.at_shader_location(3),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
