use bevy::{app::PluginGroupBuilder, camera_controller::free_camera::FreeCameraPlugin, prelude::*};
use vx_climate::ClimatePlugin;
use vx_entity::EntityPlugin;
use vx_meshing::MeshingPlugins;
use vx_mod::CoreModPlugin;
use vx_mod_behavior::ModBehaviorPlugin;
use vx_streaming::StreamingPlugin;
use vx_world::WorldPlugins;
use vx_worldgen::WorldgenPlugin;

mod debug;
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
            .add_group(debug::Plugins)
            .add_group(MeshingPlugins)
            .add(LocalPlayerPlugin::default())
            .add(EntityPlugin)
            .add(FreeCameraPlugin)
            .add(CoreModPlugin)
            .add(ModBehaviorPlugin)
            .add(ClimatePlugin)
            .add(StreamingPlugin)
            .add(WorldgenPlugin)
    }
}
