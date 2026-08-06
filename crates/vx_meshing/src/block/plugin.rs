use bevy::prelude::*;
use vx_mod_resource::{loader, registry};

use crate::block::{self, BlockMaterial, material::SharedBlockMaterial};

#[derive(Default)]
pub struct BlockMeshingPlugin;

impl BlockMeshingPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`If<Res<BlockTextureArray>>` must be passed by value as is required by bevy"
    )]
    fn setup_block_material(
        block_texture_array: If<Res<block::TextureArray>>,
        mut materials: ResMut<Assets<BlockMaterial>>,
        mut commands: Commands,
    ) {
        let handle = materials.add(BlockMaterial {
            texture: block_texture_array.texture_handle().clone(),
        });

        commands.insert_resource(SharedBlockMaterial::new(handle));
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<vx_mod_resource::Registry<loader::TextureLoader>>`,
            `Res<AssetServer>`, and `Res<Assets<Image>>` must be passed by value
            as is required by bevy
        "
    )]
    fn setup_block_texture_array(
        texture_registry: Res<vx_mod_resource::Registry<loader::TextureLoader>>,
        asset_server: Res<AssetServer>,
        images: Res<Assets<Image>>,
        mut commands: Commands,
    ) {
        let texture_array = block::TextureArray::build(&texture_registry, &asset_server, &images);
        commands.insert_resource(texture_array);
    }
}

impl Plugin for BlockMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlockMaterial>::default())
            .add_systems(
                PreUpdate,
                Self::setup_block_texture_array
                    .run_if(resource_added::<registry::Ready<loader::TextureLoader>>),
            )
            .add_systems(
                Update,
                Self::setup_block_material.run_if(not(resource_exists::<SharedBlockMaterial>)),
            );
    }
}
