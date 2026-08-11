use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct Config {
    load_radius: u32,
    unload_radius: u32,
}

impl Config {
    #[must_use]
    pub const fn load_radius(self) -> u32 {
        self.load_radius
    }

    #[must_use]
    pub const fn unload_radius(self) -> u32 {
        self.unload_radius
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            load_radius: 16,
            unload_radius: 18,
        }
    }
}
