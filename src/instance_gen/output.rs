use serde::{Deserialize, Serialize};

use common::agent::agent::Agent;

use crate::args::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutSettings {
    id: String,
    kind: String,
    greedy: bool,
    seed: u64,
    grid: OutGridSettings,
    aux_path: Option<String>,
    agents: OutAgentsSettings,
    time_max: usize,
    init: (usize, usize),
    goal: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutGridSettings {
    width: usize,
    height: usize,
    obstacles: usize,
    noise: Option<OutNoiseSettings>,
    custom: Option<Vec<(usize, usize)>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutNoiseSettings {
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
    amplitude: f64,
    frequency: f64,
    cell_size: usize,
    val_limit: u32,
    cell_limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutAgentsSettings {
    paths: Vec<Vec<(usize, usize)>>,
}

impl OutSettings {
    pub fn new(agents: &Vec<Agent>, cfg: &Config, init: (usize, usize), goal: (usize, usize), limit: u32, limit_cell: usize) -> Self {
        return OutSettings {
            seed: cfg.seed,
            id: cfg.id.clone(),
            kind: "instance".to_string(),
            aux_path: cfg.aux_path.as_ref().cloned(),
            time_max: cfg.time_max,
            greedy: cfg.greedy,
            init,
            goal,
            grid: OutGridSettings {
                width: cfg.size.0,
                height: cfg.size.1,
                obstacles: cfg.obstacles,
                noise: Some(OutNoiseSettings {
                    octaves: cfg.noise_params.octaves.unwrap_or(1),
                    persistence: cfg.noise_params.persistence.unwrap_or(0.5),
                    lacunarity: cfg.noise_params.lacunarity.unwrap_or(2.0),
                    amplitude: cfg.noise_params.amplitude.unwrap_or(1.0),
                    frequency: cfg.noise_params.frequency.unwrap_or(1.0),
                    cell_size: cfg.noise_params.cell_size.unwrap_or(100),
                    val_limit: limit,
                    cell_limit: limit_cell,
                }),
                custom: None,
            },
            agents: OutAgentsSettings {
                paths: agents.iter().map(|x| x.get_moves().clone()).collect::<Vec<Vec<(usize, usize)>>>()
            },
        };
    }
}