use bevy::prelude::*;

use crate::player::LocalPlayer;

#[derive(Default, Debug)]
pub struct LocalPlayerPlugin {
    player_name: String,
}

#[derive(Resource)]
struct PlayerName(String);

impl LocalPlayerPlugin {
    #[must_use]
    pub const fn new(player_name: String) -> Self {
        Self { player_name }
    }

    fn spawn_local_player(mut commands: Commands, player_name: Res<PlayerName>) {
        commands.spawn((LocalPlayer, Name::new(player_name.0.clone())));
    }
}

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerName(self.player_name.clone()))
            .add_systems(
                Startup,
                Self::spawn_local_player.run_if(resource_added::<PlayerName>),
            );
    }
}
