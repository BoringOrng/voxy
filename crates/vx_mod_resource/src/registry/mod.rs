use bevy::{platform::collections::HashMap, prelude::*};

mod plugin;
mod ready;

pub use plugin::LoaderPlugin;
pub use ready::Ready;

#[derive(Resource, Deref, DerefMut)]
pub struct Registry<L: crate::Loader>(HashMap<String, L::Asset>);

impl<L: crate::Loader> Registry<L> {
    pub(crate) const fn new(map: HashMap<String, L::Asset>) -> Self {
        Self(map)
    }
}
