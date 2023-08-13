use std::collections::HashMap;
use std::fs::File;
use bincode::error::DecodeError;
use flate2::read::ZlibDecoder;
use common::agent::agent::Agent;
use common::agent::agent_manager::AgentManager;
use common::field::field::{CustomField, Field, InstanceField, RandomField};
use common::noise::perlin::PerlinNoise;
use crate::args::Config;

mod args;

type AuxMap = HashMap<(usize, usize), (f64, Option<(usize, usize)>)>;

fn create_field_from_configs(cfg: &Config) -> Result<InstanceField, &str>{
    if let Some(noise) = cfg.grid.noise.as_ref() {
        let p_noise = PerlinNoise::new(
            Some(cfg.seed),
            Some(noise.octaves),
            Some(noise.persistence),
            Some(noise.lacunarity),
            Some(noise.amplitude),
            Some(noise.frequency),
            Some(noise.cell_size)
        );

        return Ok(RandomField::new(p_noise, noise.val_limit, noise.cell_limit, (cfg.grid.width, cfg.grid.height), cfg.grid.obstacles));
    }

    if let Some(obstacles) = cfg.grid.custom.clone() {
        return Ok(CustomField::new(cfg.seed, (cfg.grid.width, cfg.grid.height), obstacles))
    }

    return Err("Cannot load the field. neither noise nor custom are defined in the settings")
}

fn load_aux(path: &str) -> Result<AuxMap, DecodeError>{
    let f = File::open(path).expect("Cannot open file");
    let mut zlib = ZlibDecoder::new(f);
    let config = bincode::config::standard();
    return bincode::decode_from_std_read(&mut zlib, config);
}

fn solve(field: &InstanceField, agents: &AgentManager, init: (usize, usize), goal: (usize, usize), tmax: usize, aux: Option<AuxMap>) -> () {

}

fn main() {
    let cfg = Config::load();

    // First of all create the field
    let field = create_field_from_configs(&cfg).expect("Cannot create field");

    //then create the agents
    let mut agents = Vec::with_capacity(cfg.agents.paths.len());
    for a in cfg.agents.paths{
        agents.push(Agent::from(a));
    }
    let mgr = AgentManager::new(agents);

    //load aux if present
    let mut aux: Option<AuxMap> = None;
    if let Some(path) = cfg.aux_path{
        aux = Some(load_aux(path.as_str()).expect("Decode Error"))
    }
}