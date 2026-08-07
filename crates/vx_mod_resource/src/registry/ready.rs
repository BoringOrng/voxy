use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Ready<L: crate::Loader>(PhantomData<L>);

impl<L: crate::Loader> Default for Ready<L> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}
