use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::field::field::InstanceField;

pub struct Agent {
    moves: Vec<(usize, usize)>,
    rng: Xoshiro256PlusPlus,
    stopped: bool,
}

impl Agent {
    pub fn new(seed: u64, start_pos: (usize, usize)) -> Self {
        return Agent {
            moves: vec![start_pos],
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
            stopped: false,
        };
    }

    pub fn from(moves: Vec<(usize, usize)>) -> Self {
        return Agent {
            moves,
            rng: Xoshiro256PlusPlus::seed_from_u64(42), //we do not need it
            stopped: true,
        };
    }

    pub fn get_pos(&self, time: usize) -> (usize, usize) {
        if time >= self.moves.len() {
            return self.get_last_pos();
        }
        return self.moves.get(time).cloned().unwrap();
    }

    pub fn get_last_pos(&self) -> (usize, usize) {
        return self.moves.last().cloned().unwrap();
    }

    pub fn next_move(&mut self, field: &InstanceField, others: Vec<(usize, usize)>, stop_prob: f64) {
        if self.stopped {
            return;
        }

        let pos = self.get_last_pos();
        let avail_moves: Vec<(usize, usize)> = field.iter_neighbors(pos.0, pos.1)
            .filter(|x| !others.contains(x))
            .collect::<Vec<(usize, usize)>>();

        let mv: (usize, usize) = avail_moves.get(self.rng.next_u64() as usize % avail_moves.len()).cloned().unwrap();
        self.moves.push(mv);

        if (self.rng.next_u32() as f64 / (u32::MAX as f64)) < stop_prob {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn get_moves(&self) -> &Vec<(usize, usize)> {
        return &self.moves;
    }
}

pub fn get_agents_at_time(agents: &Vec<Agent>, time: usize, exclude: Option<(usize, usize)>) -> Vec<(usize, usize)> {
    return agents.iter()
        .map(|x| x.get_pos(time))
        .filter(|x| Some(x).cloned() != exclude)
        .collect::<Vec<(usize, usize)>>();
}

pub fn get_agents_last(agents: &Vec<Agent>, exclude: Option<(usize, usize)>) -> Vec<(usize, usize)> {
    return agents.iter()
        .map(|x| x.get_last_pos())
        .filter(|x| Some(x).cloned() != exclude)
        .collect::<Vec<(usize, usize)>>();
}
