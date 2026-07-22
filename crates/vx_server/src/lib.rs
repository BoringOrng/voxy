use std::time::Duration;

use bevy::{
    app::{PluginGroupBuilder, ScheduleRunnerPlugin},
    prelude::*,
};

#[derive(Default)]
pub struct VxServerPlugins;

impl PluginGroup for VxServerPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(MinimalPlugins)
            .add(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1.0 / 40.0,
            )))
    }
}
