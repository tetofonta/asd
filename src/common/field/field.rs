use std::fmt::{Display, Formatter};
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::field::neighbor_iterator::NeighborIterator;
use crate::noise::perlin::PerlinNoise;

pub struct Field {
    val_limit: u32,
    cell_limit: usize,
    width: usize,
    height: usize,

    rng: Xoshiro256PlusPlus,
    field_noise: PerlinNoise,
}

impl Field {
    pub fn new(noise: PerlinNoise, val_limit: u32, cell_limit: usize, size: (usize, usize)) -> Self {
        return Field {
            val_limit,
            cell_limit,
            width: size.0,
            height: size.1,
            rng: Xoshiro256PlusPlus::seed_from_u64(noise.get_seed()),
            field_noise: noise,
        };
    }

    pub fn is_obstacle(&self, x: usize, y: usize) -> bool {
        let val = self.field_noise.get_noise_u32(x, y);
        return val < self.val_limit || (val == self.val_limit && y * self.width + x <= self.cell_limit);
    }

    pub fn rnd_pick(&mut self, occupied: &Vec<(usize, usize)>) -> Result<(usize, usize), ()>{
        let mut x = (self.rng.next_u64() % self.width as u64 ) as usize;
        let mut y = (self.rng.next_u64() % self.height as u64) as usize;
        let mut times = 0;
        while self.is_obstacle(x, y) || occupied.contains(&(x, y)){
            x = (x + 1) % self.width;
            if x == 0 {
                y = (y + 1) % self.width;
            }
            times += 1;
            if times >= self.width * self.height{
                return Err(());
            }
        }
        return Ok((x, y));
    }

    pub fn iter_neighbors(&self, x: usize, y: usize) -> NeighborIterator{
        return NeighborIterator::new(&self, (x, y))
    }

    pub fn exists(&self, x: usize, y: usize) -> bool{
        return x < self.width && y < self.height;
    }
}


impl Display for Field{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for y in 0..self.height{
            for x in 0..self.width{
                s.push(if self.is_obstacle(x, y) { '#' } else {'.'});
            }
            s.push('\n');
        }
        write!(f, "Field({}x{}), VAL_LIMIT: {}, CELL_LIMIT: {}\n{}", self.width, self.height, self.val_limit, self.cell_limit, s)
    }
}