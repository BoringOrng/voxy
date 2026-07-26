use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Clone, Debug, Default, Deref, DerefMut, Resource)]
pub struct TextureRegistry(HashMap<String, Handle<Image>>);
