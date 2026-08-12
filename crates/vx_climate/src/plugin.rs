use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::{self, SystemTime},
};

use bevy::prelude::*;
use noiz::rng::NoiseRng;

pub struct ClimatePlugin;

impl ClimatePlugin {
    fn generate_seed(mut commands: Commands) {
        let t = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("You're far from home, traveler");

        let mut hasher = DefaultHasher::new();
        t.as_nanos().hash(&mut hasher);

        let hashed_t = hasher.finish();
        let lo = (hashed_t & 0xFFFF_FFFF) as u32;
        let hi = (hashed_t >> 32) as u32;

        let mut rng = NoiseRng(lo ^ hi);

        let lo = u64::from(rng.rand_u32(u32::from_ne_bytes(*b"voxy")));
        rng.re_seed();
        let hi = u64::from(rng.rand_u32(u32::from_be_bytes(*b"voxy")));

        commands.insert_resource(super::WorldSeed((hi << 32) | lo));
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<super::WorldSeed>` must be passed by value as is required by bevy"
    )]
    fn setup_sampler(mut commands: Commands, seed: Res<super::WorldSeed>) {
        let mut sampler = super::Sampler::default();
        sampler.reseed(*seed);

        commands.insert_resource(sampler);
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<super::WorldSeed>` must be passed by value as is required by bevy"
    )]
    fn log_seed(world_seed: Res<super::WorldSeed>) {
        info!("Running voxy with world seed: `{}`", **world_seed);
    }
}

impl Plugin for ClimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            Self::generate_seed.run_if(not(resource_exists::<super::WorldSeed>)),
        )
        .add_systems(
            Update,
            (Self::log_seed, Self::setup_sampler).run_if(resource_added::<super::WorldSeed>),
        );
    }
}
