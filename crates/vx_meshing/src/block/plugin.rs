use bevy::prelude::*;
use vx_mod_resource::BlockTextureArray;

use crate::block::{BlockMaterial, material::SharedBlockMaterial};

#[derive(Default)]
pub struct BlockMeshingPlugin;

impl BlockMeshingPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`If<Res<BlockTextureArray>>` must be passed by value as is required by bevy"
    )]
    fn setup_block_material(
        block_texture_array: If<Res<BlockTextureArray>>,
        mut materials: ResMut<Assets<BlockMaterial>>,
        mut commands: Commands,
    ) {
        let handle = materials.add(BlockMaterial {
            texture: block_texture_array.texture_handle().clone(),
        });

        commands.insert_resource(SharedBlockMaterial::new(handle));
    }
}

impl Plugin for BlockMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlockMaterial>::default())
            .add_systems(
                Update,
                Self::setup_block_material.run_if(not(resource_exists::<SharedBlockMaterial>)),
            );
    }
}
