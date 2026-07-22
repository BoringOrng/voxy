use bevy::prelude::*;
use vx_player::Player;

mod plugin;

pub use plugin::LocalPlayerPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
#[require(Player, Camera3d)]
pub struct LocalPlayer;
