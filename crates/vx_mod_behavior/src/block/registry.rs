use bevy::{platform::collections::HashMap, prelude::*};

use crate::block::Block;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct BlockRegistry(HashMap<String, Block>);
