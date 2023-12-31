use std::collections::{BTreeSet, HashSet};
use std::fmt::{Display, Formatter};

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::field::neighbor_iterator::NeighborIterator;
use crate::noise::perlin::PerlinNoise;

pub enum InstanceField {
    RandomField(RandomField),
    CustomField(CustomField),
}

impl Field for InstanceField {
    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        return match self {
            InstanceField::RandomField(e) => e.is_obstacle(x, y),
            InstanceField::CustomField(e) => e.is_obstacle(x, y)
        };
    }

    fn obstacles(&self) -> usize {
        return match self {
            InstanceField::RandomField(e) => e.obstacles(),
            InstanceField::CustomField(e) => e.obstacles()
        };
    }

    fn width(&self) -> usize {
        return match self {
            InstanceField::RandomField(e) => e.width(),
            InstanceField::CustomField(e) => e.width()
        };
    }

    fn height(&self) -> usize {
        return match self {
            InstanceField::RandomField(e) => e.height(),
            InstanceField::CustomField(e) => e.height()
        };
    }

    fn rng(&mut self) -> &mut Xoshiro256PlusPlus {
        return match self {
            InstanceField::RandomField(e) => e.rng(),
            InstanceField::CustomField(e) => e.rng()
        };
    }
}

impl InstanceField {
    pub fn iter_neighbors(&self, x: usize, y: usize) -> NeighborIterator {
        return NeighborIterator::new(self, (x, y));
    }
}

impl Display for InstanceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            InstanceField::RandomField(e) => e.fmt(f),
            InstanceField::CustomField(e) => e.fmt(f)
        };
    }
}

pub trait Field {
    fn is_obstacle(&self, x: usize, y: usize) -> bool;
    fn obstacles(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn rng(&mut self) -> &mut Xoshiro256PlusPlus;
    fn rnd_pick(&mut self, occupied: &HashSet<(usize, usize)>) -> Result<(usize, usize), ()> {
        let mut x = (self.rng().next_u64() % self.width() as u64) as usize;
        let mut y = (self.rng().next_u64() % self.height() as u64) as usize;
        let mut times = 0;
        while self.is_obstacle(x, y) || occupied.contains(&(x, y)) {
            x = (x + 1) % self.width();
            if x == 0 {
                y = (y + 1) % self.height();
            }
            times += 1;
            if times >= self.width() * self.height() {
                return Err(());
            }
        }
        return Ok((x, y));
    }
    fn exists(&self, x: usize, y: usize) -> bool {
        return x < self.width() && y < self.height();
    }
    fn nodes(&self) -> usize {
        return self.width() * self.height() - self.obstacles();
    }
}

pub struct RandomField {
    val_limit: u32,
    cell_limit: usize,
    width: usize,
    height: usize,
    obstacles: usize,

    rng: Xoshiro256PlusPlus,
    field_noise: PerlinNoise,
}

impl RandomField {
    pub fn new(noise: PerlinNoise, val_limit: u32, cell_limit: usize, size: (usize, usize), obstacles: usize) -> InstanceField {
        return InstanceField::RandomField(RandomField {
            val_limit,
            cell_limit,
            width: size.0,
            height: size.1,
            rng: Xoshiro256PlusPlus::seed_from_u64(noise.get_seed()),
            field_noise: noise,
            obstacles,
        });
    }
}

impl Field for RandomField {
    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        let val = self.field_noise.get_noise_u32(x, y);
        return val < self.val_limit || (val == self.val_limit && y * self.width + x <= self.cell_limit);
    }

    fn obstacles(&self) -> usize {
        return self.obstacles;
    }

    fn width(&self) -> usize {
        return self.width;
    }

    fn height(&self) -> usize {
        return self.height;
    }

    fn rng(&mut self) -> &mut Xoshiro256PlusPlus {
        return &mut self.rng;
    }
}

impl Display for RandomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(if self.is_obstacle(x, y) { '#' } else { '.' });
            }
            s.push('\n');
        }
        write!(f, "Field({}x{}), VAL_LIMIT: {}, CELL_LIMIT: {}\n{}", self.width, self.height, self.val_limit, self.cell_limit, s)
    }
}

pub struct CustomField {
    width: usize,
    height: usize,
    rng: Xoshiro256PlusPlus,
    obstacles: HashSet<(usize, usize)>,
}

impl CustomField {
    pub fn new(seed: u64, size: (usize, usize), obstacles: Vec<(usize, usize)>) -> InstanceField {
        return InstanceField::CustomField(CustomField {
            width: size.0,
            height: size.1,
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
            obstacles: HashSet::from_iter(obstacles),
        });
    }
}

impl Field for CustomField {
    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        return self.obstacles.contains(&(x, y));
    }

    fn obstacles(&self) -> usize {
        return self.obstacles.len();
    }

    fn width(&self) -> usize {
        return self.width;
    }

    fn height(&self) -> usize {
        return self.height;
    }

    fn rng(&mut self) -> &mut Xoshiro256PlusPlus {
        return &mut self.rng;
    }
}

impl Display for CustomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(if self.is_obstacle(x, y) { '#' } else { '.' });
            }
            s.push('\n');
        }
        write!(f, "Field({}x{})\n{}", self.width, self.height, s)
    }
}