use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use common::agent::agent::{Agent, get_agents_at_time, get_agents_last};
use common::field::open_node::OpenNode;
use common::field::weight;
use common::noise::perlin::PerlinNoise;
use bincode::{config};
use common::field::field::{Field, InstanceField, RandomField};

mod args;
mod noise_value;
mod output;

use crate::args::Config;
use crate::noise_value::NoiseValue;
use crate::output::OutSettings;

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

fn gen_agents(cfg: &Config, field: &mut InstanceField) -> Vec<Agent>{
    let mut agents: Vec<Agent> = Vec::with_capacity(cfg.agents.number);
    let mut start_positions: Vec<(usize, usize)> = Vec::with_capacity(cfg.agents.number);
    for i in 0..cfg.agents.number {
        let position = field.rnd_pick(&start_positions).expect("Error during the creation of the agent");
        start_positions.push(position);
        agents.push(
            Agent::new(
                cfg.seed + i as u64,
                position,
            )
        )
    }
    for _t in 1..cfg.time_max {
        for i in 0..agents.len() {
            let moves = get_agents_last(&agents, Some(agents.get(i).unwrap().get_last_pos()));
            let a = agents.get_mut(i).unwrap();
            a.next_move(field, moves, cfg.agents.stop_probability)
        }
    }
    return agents
}

fn gen_entity_positions(field: &mut InstanceField, agents: &Vec<Agent>) -> ((usize, usize), (usize, usize)){
    let init = field.rnd_pick(&get_agents_at_time(&agents, 0, None)).expect("Cannot pick starting position. grid is occupied at time 0");
    let mut occupied_end_positions = get_agents_last(&agents,None);
    occupied_end_positions.push(init); //Theoretically we could start and end in the same position
    let goal = field.rnd_pick(&occupied_end_positions).expect("Cannot pick starting position. grid is occupied at time 0");
    return (init, goal)
}

fn compute_aux(field: &InstanceField, goal: (usize, usize), path: &str, tmax: usize) {
    let mut nodes: HashMap<(usize, usize), (f64, Option<(usize, usize)>)> = HashMap::with_capacity(field.nodes());
    let mut heap: BinaryHeap<Reverse<OpenNode<(usize, usize)>>> = BinaryHeap::new();

    nodes.insert(goal, (0.0, None));
    heap.push(Reverse(OpenNode::new(0.0, goal, 0)));
    while heap.len() > 0{
        let element = heap.pop().unwrap().0;
        if element.time() > tmax{
            continue
        }
        for adj in field.iter_neighbors(element.node().0, element.node().1){
            let cur_weight = nodes.get(element.node()).cloned().expect("Node never reached").0 + weight(element.node(), &adj);
            let (dest_weight, _) = nodes.get(&adj).cloned().unwrap_or((f64::MAX, None));
            if cur_weight < dest_weight{
                nodes.insert(adj, (cur_weight, Some(element.node()).cloned()));
                heap.push(Reverse(OpenNode::new(cur_weight, adj, element.time() + 1)));
            }
        }

    }

    //store the file
    nodes.shrink_to_fit();
    let file = File::create(path).expect("File creation error");
    let config = config::standard();
    let mut e = ZlibEncoder::new(file, Compression::best());
    bincode::encode_into_std_write(&nodes, &mut e, config).expect("Cannot serialize");
}

fn write_results(agents: &Vec<Agent>, cfg: &Config, init: (usize, usize), goal: (usize, usize), limit: u32, limit_cell: usize){
    let out = OutSettings::new(agents, cfg, init, goal, limit, limit_cell);
    serde_yaml::to_writer(std::io::stdout(), &out).unwrap();
}

fn main() {
    let cfg = Config::load(None);

    // First generate the nth minimum value coming from the noise generator seeded.
    // We need to keep track of the last occurrence of the nth-minimum cell
    let (limit, cell, noise) = gen_field_parameters(&cfg);

    //configure the field
    let mut field = RandomField::new(noise, limit, cell, cfg.size, cfg.obstacles);

    // Agents and start-finish can be recalculated based on the seed but
    // it's better to save the instance for more flexibility.
    let agents = gen_agents(&cfg, &mut field);

    //get the randomly picked start and end positions
    let (init, goal) = gen_entity_positions(&mut field, &agents);

    //precalculate the auxiliary table
    if let Some(path) = cfg.aux_path.as_ref(){
        if cfg.greedy{
            compute_aux(&field, goal, path.as_str(), cfg.time_max);
        }
    }

    write_results(&agents, &cfg, init, goal, limit, cell);
    eprintln!("{}", field);
}