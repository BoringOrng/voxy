use bevy::prelude::*;
use noiz::prelude::*;

type WindNoise =
    Noise<BlendCellValues<Voronoi, DistanceBlend<ManhattanLength>, Random<UNorm, f32>>>;

type TemperatureNoise = Noise<
    LayeredNoise<
        NormedByDerivative<f32, EuclideanLength, PeakDerivativeContribution>,
        Persistence,
        FractalLayers<Octave<common_noise::PerlinWithDerivative>>,
    >,
>;

type HumidityNoise =
    Noise<LayeredNoise<Normed<f32>, Persistence, FractalLayers<Octave<common_noise::Perlin>>>>;

type ContinentalNoise = Noise<
    LayeredNoise<
        NormedByDerivative<f32, EuclideanLength, PeakDerivativeContribution>,
        Persistence,
        FractalLayers<Octave<common_noise::PerlinWithDerivative>>,
    >,
>;

type AgeNoise = Noise<
    LayeredNoise<
        NormedByDerivative<f32, EuclideanLength, PeakDerivativeContribution>,
        Persistence,
        FractalLayers<Octave<common_noise::SimplexWithDerivative>>,
    >,
>;

#[derive(Resource, Clone)]
pub struct Sampler {
    wind: WindNoise,
    temperature: TemperatureNoise,
    humidity: HumidityNoise,
    continental: ContinentalNoise,
    age: AgeNoise,
}

impl Sampler {
    pub fn reseed(&mut self, seed: super::WorldSeed) {
        // ignore this for now I'm too lazy to change the seed to an u32.
        let seed = seed.wrapping_add(u64::from_le_bytes(*b"voxyorng"));
        let seed = ((seed >> 32) as u32).wrapping_add((seed & 0xFFFF_FFFF) as u32);

        self.wind.set_seed(seed);
        self.temperature.set_seed(seed.wrapping_add(1000));
        self.humidity.set_seed(seed.wrapping_add(2000));
        self.continental.set_seed(seed.wrapping_add(3000));
        self.age.set_seed(seed.wrapping_add(4000));
    }

    #[must_use]
    #[inline]
    pub fn wind(&self, pos: Vec2) -> f32 {
        self.wind.sample_for::<f32>(pos)
    }

    #[must_use]
    #[inline]
    pub fn temperature(&self, pos: Vec2) -> f32 {
        self.temperature.sample_for::<f32>(pos)
    }

    #[must_use]
    #[inline]
    pub fn humidity(&self, pos: Vec2) -> f32 {
        self.humidity.sample_for::<f32>(pos)
    }

    #[must_use]
    #[inline]
    pub fn continental(&self, pos: Vec2) -> f32 {
        self.continental.sample_for::<f32>(pos) * 256.0
    }

    #[must_use]
    #[inline]
    pub fn age(&self, pos: Vec2) -> f32 {
        self.age.sample_for::<f32>(pos)
    }
}

impl Default for Sampler {
    fn default() -> Self {
        let mut sampler = Self {
            wind: default(),
            temperature: default(),
            humidity: default(),
            continental: Noise::from(LayeredNoise::new(
                NormedByDerivative::<f32, EuclideanLength, PeakDerivativeContribution>::default()
                    .with_falloff(0.3),
                Persistence(0.6),
                FractalLayers {
                    layer: Octave::<common_noise::PerlinWithDerivative>::default(),
                    lacunarity: 1.8,
                    amount: 8,
                },
            )),
            age: default(),
        };

        sampler.continental.set_period(400.0);

        sampler
    }
}
