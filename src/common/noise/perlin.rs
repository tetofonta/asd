//! 2D Perin noise generation functions
//!
//! This package allows to calculate the value of perlin noise
//! at a given coordinate.
//!
//! Lattice vectors are generating as described in https://mrl.cs.nyu.edu/~perlin/paper445.pdf
//! Param description https://gamedev.stackexchange.com/questions/197861/how-to-handle-octave-frequency-in-the-perlin-noise-algorithm

use std::collections::hash_map::DefaultHasher;
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::ops;
use std::time::SystemTime;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct PerlinNoise {
    seed: u64,

    octaves: usize,
    persistence: f64,
    lacunarity: f64,
    amplitude: f64,
    frequency: f64,
    cell_size: usize,
}

#[derive(Clone, Copy)]
struct Vec2(f64, f64);

impl ops::Mul for Vec2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        return self.0 * rhs.0 + self.1 * rhs.1;
    }
}


impl PerlinNoise {
    /// Creates a new instance of the perlin noise generator
    pub fn new(seed: Option<u64>, octaves: Option<usize>, persistence: Option<f64>, lacunarity: Option<f64>, amplitude: Option<f64>, frequency: Option<f64>, cell_size: Option<usize>) -> Self {
        return PerlinNoise {
            seed: seed.unwrap_or(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros() as u64),
            octaves: octaves.unwrap_or(1),
            persistence: persistence.unwrap_or(0.5),
            lacunarity: lacunarity.unwrap_or(2.0),
            amplitude: amplitude.unwrap_or(1.0),
            frequency: frequency.unwrap_or(1.0),
            cell_size: cell_size.unwrap_or(100),
        };
    }

    fn get_rand_vect(&self, x: usize, y: usize) -> Vec2 {
        let mut s = DefaultHasher::new();
        (self.seed, x, y).hash(&mut s);
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(s.finish());
        let angl: f64 = ((rng.next_u32() as f64) / u32::MAX as f64) * 2.0 * PI;
        return Vec2(angl.cos(), angl.sin());
    }

    fn ease(a: f64, b: f64, c: f64, d: f64, ox: f64, oy: f64) -> f64 {
        let u = (b - a) * (3.0 - ox * 2.0) * ox * ox + a;
        let v = (d - c) * (3.0 - ox * 2.0) * ox * ox + c;
        return (v - u) * (3.0 - oy * 2.0) * oy * oy + u;
    }

    fn get(&self, x: f64, y: f64) -> f64 {
        let base_x = x.trunc() as usize;
        let base_y = y.trunc() as usize;
        let offset_x = x.fract();
        let offset_y = y.fract();

        return PerlinNoise::ease(
            self.get_rand_vect(base_x, base_y) * Vec2(offset_x, offset_y),
            self.get_rand_vect(base_x + 1, base_y) * Vec2(1.0 - offset_x, offset_y),
            self.get_rand_vect(base_x, base_y + 1) * Vec2(offset_x, 1.0 - offset_y),
            self.get_rand_vect(base_x + 1, base_y + 1) * Vec2(1.0 - offset_x, 1.0 - offset_y),
            offset_x,
            offset_y,
        );
    }

    pub fn gen_noise(&self, x: usize, y: usize) -> f64 {
        let xx = x as f64 / self.cell_size as f64;
        let yy = y as f64 / self.cell_size as f64;

        let mut val: f64 = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;

        for _ in 0..self.octaves {
            val += amp * self.get(xx * freq, yy * freq);
            amp *= self.persistence;
            freq *= self.lacunarity;
        }

        return val;
    }

    pub fn get_noise_u32(&self, x: usize, y: usize) -> u32 {
        return (self.gen_normalized(x, y) * (u32::MAX as f64)) as u32;
    }

    pub fn gen_normalized(&self, x: usize, y: usize) -> f64 {
        return (self.gen_noise(x, y) / (self.octaves as f64) + 1.0) / 2.0;
    }

    pub fn get_seed(&self) -> u64 {
        return self.seed;
    }
}