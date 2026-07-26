use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum BlockLoadState {
    #[default]
    Loading,
    Loaded,
}
