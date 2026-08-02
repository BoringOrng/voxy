use bevy::prelude::*;

use crate::ModInfo;

#[derive(Debug, Deref, DerefMut, Default, Resource)]
pub struct LoadedMods(Vec<ModInfo>);

impl LoadedMods {
    pub(crate) const fn new(mods: Vec<ModInfo>) -> Self {
        Self(mods)
    }
}
