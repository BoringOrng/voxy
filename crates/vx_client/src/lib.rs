use bevy::{app::PluginGroupBuilder, camera_controller::free_camera::FreeCameraPlugin, prelude::*};
use vx_meshing::{MeshingPlugins, chunk::DirtyChunk};
use vx_mod::CoreModPlugin;
use vx_mod_behavior::ModBehaviorPlugin;
use vx_mod_resource::ModResourcePlugin;
use vx_world::{
    WorldPlugins,
    block::{BlockId, BlockPos},
    chunk::ChunkData,
};

pub mod player;

use player::LocalPlayerPlugin;

#[derive(Default)]
pub struct VxClientPlugins;

impl PluginGroup for VxClientPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins)
            .add_group(WorldPlugins)
            .add_group(vx_mod_resource::Plugins)
            .add_group(MeshingPlugins)
            .add(LocalPlayerPlugin::default())
            .add(FreeCameraPlugin)
            .add(CoreModPlugin)
            .add(ModBehaviorPlugin)
            .add(ModResourcePlugin)
            .add(TempPlugin)
    }
}

struct TempPlugin;

impl Plugin for TempPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_temp_mesh);
    }
}

fn spawn_temp_mesh(mut commands: Commands) {
    let mut chunk_data = ChunkData::default();
    chunk_data.insert(
        BlockPos::new(0, 0, 0),
        BlockId::from_raw(2.try_into().expect("two isn't zero")),
    );
    chunk_data.insert(
        BlockPos::new(16, 16, 16),
        BlockId::from_raw(1.try_into().expect("one isn't zero")),
    );
    chunk_data.insert(
        BlockPos::new(17, 16, 16),
        BlockId::from_raw(2.try_into().expect("two isn't zero")),
    );
    chunk_data.insert(
        BlockPos::new(16, 17, 16),
        BlockId::from_raw(2.try_into().expect("two isn't zero")),
    );

    commands.spawn((DirtyChunk, chunk_data));
}
