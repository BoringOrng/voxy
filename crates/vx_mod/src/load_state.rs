use bevy::prelude::*;

#[derive(Clone, Copy, States, PartialEq, Eq, Hash, Debug, Default)]
pub enum ModLoadState {
    #[default]
    Loading,
    Loaded,
}
