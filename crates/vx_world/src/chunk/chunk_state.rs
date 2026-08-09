use bevy::prelude::*;

use super::Chunk;

#[derive(Component)]
#[require(Chunk)]
pub struct NeedsMeshing;

#[derive(Component)]
#[require(Chunk)]
pub struct NeedsWorldgen;
