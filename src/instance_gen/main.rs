use std::collections::BinaryHeap;
use std::hash::Hash;
use common::agent::agent::{Agent, get_agents_at_time, get_agents_last};
use common::field::field::Field;
use common::noise::perlin::PerlinNoise;

mod args;
mod noise_value;

use crate::args::Config;
use crate::noise_value::NoiseValue;


fn gen_field_parameters(cfg: &Config) -> (u32, usize, PerlinNoise) {
    let mut heap: BinaryHeap<NoiseValue> = BinaryHeap::with_capacity(cfg.obstacles);
    let noise = PerlinNoise::new(Some(cfg.seed), cfg.noise_params.octaves, cfg.noise_params.persistence, cfg.noise_params.lacunarity, cfg.noise_params.amplitude, cfg.noise_params.frequency, cfg.noise_params.cell_size);

    for y in 0..cfg.size.0 {
        for x in 0..cfg.size.1 {
            let val = NoiseValue {
                value: noise.get_noise_u32(x, y),
                cell: (x, y),
            };
            if heap.len() < cfg.obstacles {
                heap.push(val)
            } else if heap.peek().unwrap().value > val.value {
                heap.pop();
                heap.push(val)
            }
        }
    }

    let v = heap.peek().unwrap();
    return (v.value, cfg.size.0 * v.cell.1 + v.cell.0, noise);
}

fn main() {
    let cfg = Config::load(None);

    // First generate the nth minimum value coming from the noise generator seeded.
    // We need to keep track of the last occurrence of the nth-minimum cell
    let (limit, cell, noise) = gen_field_parameters(&cfg);

    //configure the field
    let mut field = Field::new(noise, limit, cell, cfg.size);

    // Agents and start-finish can be recalculated based on the seed but
    // it's better to save the instance for more flexibility.
    // todo review this code because is not efficient.
    let mut agents: Vec<Agent> = Vec::with_capacity(cfg.agents.number);
    for i in 0..cfg.agents.number {
        agents.push(
            Agent::new(
                cfg.seed + i as u64,
                field.rnd_pick(&get_agents_at_time(&agents, 0, None)).expect("Error during the creation of the agent"),
            )
        )
    }
    for t in 0..cfg.time_max {
        for i in 0..agents.len() {
            let moves = get_agents_last(&agents, Some(agents.get(i).unwrap().get_last_pos()));
            let a = agents.get_mut(i).unwrap();
            a.next_move(&field, moves, cfg.agents.stop_probability)
        }
    }

    let mut occupied = get_agents_last(&agents, None);
    let init = field.rnd_pick(&occupied).expect("Cannot pick init cell");
    occupied.push(init);
    let goal = field.rnd_pick(&occupied).expect("Cannot pick goal cell");

    println!("{}, {}, {:?}, {:?}", limit, cell, init, goal);
    println!("{}", field);

    for a in &agents{
        for m in a.get_moves(){
            print!("{:?}->", m);
        }
        println!();
    }
}