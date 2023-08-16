use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;

use bincode::error::DecodeError;
use flate2::read::ZlibDecoder;

use common::agent::agent::Agent;
use common::agent::agent_manager::AgentManager;
use common::field::{heuristic, weight};
use common::field::field::{CustomField, InstanceField, RandomField};
use common::field::open_node::OpenNode;
use common::field::visited_node::VisitedNode;
use common::noise::perlin::PerlinNoise;

use crate::args::Config;
use crate::output::{Solution, SolutionPath};

mod args;
mod output;

type AuxMap = HashMap<(usize, usize), (f64, Option<(usize, usize)>)>;

fn create_field_from_configs(cfg: &Config) -> Result<InstanceField, &str> {
    if let Some(noise) = cfg.grid.noise.as_ref() {
        let p_noise = PerlinNoise::new(
            Some(cfg.seed),
            Some(noise.octaves),
            Some(noise.persistence),
            Some(noise.lacunarity),
            Some(noise.amplitude),
            Some(noise.frequency),
            Some(noise.cell_size),
            Some(noise.offset)
        );

        return Ok(RandomField::new(p_noise, noise.val_limit, noise.cell_limit, (cfg.grid.width, cfg.grid.height), cfg.grid.obstacles));
    }

    if let Some(obstacles) = cfg.grid.custom.clone() {
        return Ok(CustomField::new(cfg.seed, (cfg.grid.width, cfg.grid.height), obstacles));
    }

    return Err("Cannot load the field. neither noise nor custom are defined in the settings");
}

fn load_aux(path: &str) -> Result<AuxMap, DecodeError> {
    let f = File::open(path).expect("Cannot open file");
    let mut zlib = ZlibDecoder::new(f);
    let config = bincode::config::standard();
    return bincode::decode_from_std_read(&mut zlib, config);
}

fn verify_path(path: &Vec<(usize, usize)>, t_start: usize, t_max: usize, agents: &AgentManager, goal: (usize, usize)) -> Result<(), ()> {
    if path.len() == 0 {
        return Ok(());
    }
    if path.len() == 1 {
        return if path.get(0).cloned().unwrap() == goal { Ok(()) } else { Err(()) };
    }

    let mut i = 1;
    let mut pos = path.get(0).cloned().expect("Invalid t_start");
    if !agents.can_stay(pos, t_start) {
        return Err(());
    }

    while t_start + i <= t_max {
        let dest = path.get(i).cloned().expect("Path is shorter than time delta");
        if !agents.is_traversable(pos, dest, t_start + i - 1) {
            return Err(());
        }
        if i == path.len() - 1 {
            if dest != goal {
                return Err(());
            }
            if !agents.can_stay(dest, t_start + i) {
                return Err(());
            }
            return Ok(());
        }
        i += 1;
        pos = dest;
    }

    return Err(());
}

fn reconstruct_path(nodes: &HashMap<(usize, usize), VisitedNode>, goal: (usize, usize)) -> SolutionPath {
    let mut queue = VecDeque::new();
    let mut n = nodes.get(&goal);
    let mut t = nodes.get(&goal).unwrap().best_time();
    let mut w = 0.0;
    let mut waits = 0;

    while n.is_some() {
        let nxt = n.take().unwrap();
        let cur = nxt.node();
        queue.push_front(cur);

        if let Some(parent) = nxt.parent(t) {
            if parent == cur { waits += 1; }
            w += weight(&cur, &parent);
            n = nodes.get(&parent);
            t -= 1;
        } else {
            break;
        }
    }

    return SolutionPath {
        time: queue.len() - 1,
        weight: w,
        path: Vec::from_iter(queue.iter().cloned()),
        waits,
    };
}

fn get_path_from_aux(location: (usize, usize), aux: &AuxMap) -> Result<(Vec<(usize, usize)>, f64), ()> {
    let mut path = Vec::new();
    let reference = aux.get(&location).cloned();
    if reference.is_none() {
        return Err(());
    }
    let (w, mut next) = reference.unwrap();
    while next.is_some() {
        let n = next.take().unwrap();
        path.push(n);
        next = aux.get(&n).cloned().unwrap().1;
    }
    return Ok((path, w));
}

fn solve(field: &InstanceField, agents: &AgentManager, init: (usize, usize), goal: (usize, usize), tmax: usize, aux: Option<&AuxMap>, greedy: bool) -> Solution {
    let mut open: BinaryHeap<Reverse<OpenNode<(usize, usize)>>> = BinaryHeap::new();
    let mut closed: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut nodes: HashMap<(usize, usize), VisitedNode> = HashMap::new();
    let mut expanded: usize = 0;
    let mut opened: usize = 1;

    //initialize first node
    let mut first_node = VisitedNode::new(init); //init location
    first_node.set(0, 0.0, None, agents); //set parent to none and weight 0 at time 0
    nodes.insert(init, first_node); //store the visited node
    open.push(Reverse(OpenNode::new(0.0, init, 0)));

    while open.len() > 0 {
        expanded += 1;
        let element = open.pop().expect("Cannot pop element").0;
        let node = element.node().clone();
        let src_weight = nodes.get(&node).unwrap().weight(element.time(), agents);
        closed.insert((node, element.time()));

        if node == goal {
            if greedy { break; } else { continue; }
        }
        if element.time() >= tmax { continue; }
        if let Some(aux_map) = aux.clone() {
            let path = get_path_from_aux(node, aux_map);
            match path {
                Ok(path) => {
                    let (mut path, w) = path;
                    if let Ok(_) = verify_path(&path, element.time() + 1, tmax, &agents, goal) {
                        let mut prev_path = reconstruct_path(&nodes, node);
                        prev_path.time += path.len();
                        prev_path.weight += w;
                        if path.len() > 0 {
                            prev_path.weight += weight(&node, path.first().unwrap());
                        }
                        prev_path.path.append(&mut path);

                        verify_path(&prev_path.path, 0, tmax, &agents, goal).expect("Invalid path found!");
                        return Solution {
                            kind: "solution".to_string(),
                            opened_states: opened,
                            expanded_states: expanded,
                            path_info: prev_path,
                        };
                    }
                }
                Err(_) => panic!("There's no solution.")
            }
        }

        for neighbor in field.iter_neighbors(node.0, node.1) {
            if !agents.is_traversable(node, neighbor, element.time()) { continue; }

            if !nodes.contains_key(&neighbor) {
                nodes.insert(neighbor, VisitedNode::new(neighbor));
            }
            let dest_reference = nodes.get_mut(&neighbor).unwrap();

            let weight = weight(&node, &neighbor);
            let dst_weight = dest_reference.weight(element.time() + 1, agents);
            if closed.contains(&(neighbor, element.time() + 1)) && src_weight + weight >= dst_weight { continue; }

            if src_weight + weight < dst_weight {
                dest_reference.set(element.time() + 1, src_weight + weight, Some(node), agents);
            }

            if open.iter().filter(|x| x.0.node().clone() == neighbor && x.0.time() == element.time() + 1).count() == 0 {
                opened += 1;
                open.push(Reverse(OpenNode::new(heuristic(&neighbor, &goal) + dest_reference.weight(element.time() + 1, agents), neighbor, element.time() + 1)));
            }
        }
    }

    if nodes.get(&goal).is_none() {
        panic!("There's no solution")
    }

    let path = reconstruct_path(&nodes, goal);
    verify_path(&path.path, 0, tmax, &agents, goal).expect("Invalid path found!");
    return Solution {
        kind: "solution".to_string(),
        opened_states: opened,
        expanded_states: expanded,
        path_info: path,
    };
}

fn main() {
    let cfg = Config::load();

    // First of all create the field
    let field = create_field_from_configs(&cfg).expect("Cannot create field");
    if cfg.grid.width <= 300 && cfg.grid.height <= 300{
        eprintln!("{}", field);
    }

    //then create the agents
    let mut agents = Vec::with_capacity(cfg.agents.paths.len());
    for a in cfg.agents.paths {
        agents.push(Agent::from(a));
    }
    let mgr = AgentManager::new(agents);

    //load aux if present
    let mut aux: Option<AuxMap> = None;
    if let Some(path) = cfg.aux_path {
        if cfg.greedy {
            aux = Some(load_aux(path.as_str()).expect("Decode Error"))
        }
    }

    let sol = solve(&field, &mgr, cfg.init, cfg.goal, cfg.time_max, aux.as_ref(), cfg.greedy);
    serde_yaml::to_writer(std::io::stdout(), &sol).unwrap();
    eprintln!("GREEDY: {}", cfg.greedy);
    eprintln!("Path: {:?}", sol.path_info.path);
    eprintln!("Time: {}it", sol.path_info.time);
    eprintln!("Weight: {}", sol.path_info.weight);
    eprintln!("Waits: {}", sol.path_info.waits);
    eprintln!("States (expanded)/(opened): {}/{}", sol.expanded_states, sol.opened_states);
}