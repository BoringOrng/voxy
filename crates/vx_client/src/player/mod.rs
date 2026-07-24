use bevy::{camera_controller::free_camera::FreeCamera, prelude::*};
use vx_player::Player;

mod plugin;

pub use plugin::LocalPlayerPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
#[require(Player, Camera3d, FreeCamera)]
pub struct LocalPlayer;
