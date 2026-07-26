use bevy::prelude::*;

use crate::ModInfo;

#[derive(Debug, Deref, DerefMut, Default, Resource)]
pub struct LoadedMods(Vec<ModInfo>);
