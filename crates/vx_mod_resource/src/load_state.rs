use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum TextureLoadState {
    #[default]
    Loading,
    Loaded,
}
