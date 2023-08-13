use std::cmp::max;
use std::fs;
use std::time::SystemTime;
use argparse::{ArgumentParser, Store, StoreOption};
use yaml_rust::{YamlLoader};

#[derive(Debug)]
pub struct NoiseParams{
    pub octaves: Option<usize>,
    pub persistence: Option<f64>,
    pub lacunarity: Option<f64>,
    pub amplitude: Option<f64>,
    pub frequency: Option<f64>,
    pub cell_size: Option<usize>
}

#[derive(Debug)]
pub struct AgentParams{
    pub number: usize,
    pub stop_probability: f64
}

#[derive(Debug)]
pub struct Config{
    pub id: String,
    pub size: (usize, usize),
    pub seed: u64,

    pub noise_params: NoiseParams,
    pub agents: AgentParams,
    pub obstacles: usize,
    pub time_max: usize,

    pub aux_path: Option<String>
}

impl Config{
    pub fn load(file: Option<String>) -> Self{
        if let Some(file) = file{
            return Config::load_from_file(file, None);
        }

        let mut cfg = Config::defaults();
        let mut fname: Option<String> = None;
        let mut conf_id: Option<String> = None;
        let mut w: Option<usize> = None;
        let mut h: Option<usize> = None;

        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Instance Generator");

            ap.refer(&mut fname).add_option(&["-c", "--config"], StoreOption, "Config file name. If present configuration will be loaded from file INSTEAD of cmdline.");
            ap.refer(&mut conf_id).add_option(&["-i", "--config-id"], StoreOption, "Config ID. Allows to load one doc from the yaml file");
            ap.refer(&mut cfg.aux_path).add_option(&["-o", "--aux-file"], StoreOption, "Output aux file path");

            ap.refer(&mut cfg.seed).add_option(&["-s", "--seed"], Store, "RNG Seed");
            ap.refer(&mut w).add_option(&["-w", "--width"], StoreOption, "Grid Width");
            ap.refer(&mut h).add_option(&["-h", "--height"], StoreOption, "Grid Height");
            ap.refer(&mut cfg.obstacles).add_option(&["-o", "--obstacles"], Store, "Number of obstacles");
            ap.refer(&mut cfg.time_max).add_option(&["-t", "--tmax"], Store, "Maximum resolution time depth");

            ap.refer(&mut cfg.agents.number).add_option(&["-a", "--agents"], Store, "Number of agents");
            ap.refer(&mut cfg.agents.stop_probability).add_option(&["--agent-stop-probability"], Store, "Probability on every move for an agent to stop. [0, 1]");

            ap.refer(&mut cfg.noise_params.octaves).add_option(&["--octaves"], StoreOption, "Noise Octaves");
            ap.refer(&mut cfg.noise_params.persistence).add_option(&["--persistence"], StoreOption, "Amplitude dumping factor");
            ap.refer(&mut cfg.noise_params.lacunarity).add_option(&["--lacunarity"], StoreOption, "Frequency multiplication factor");
            ap.refer(&mut cfg.noise_params.amplitude).add_option(&["--amplitude"], StoreOption, "Starting Amplitude");
            ap.refer(&mut cfg.noise_params.frequency).add_option(&["--frequency"], StoreOption, "Starting frequency");
            ap.refer(&mut cfg.noise_params.cell_size).add_option(&["--cell_size"], StoreOption, "Cell size");
            ap.parse_args_or_exit();
        }

        if let Some(fname) = fname{
            //todo check config validity
            return Config::load_from_file(fname, conf_id);
        }
        //todo check config validity
        if w.is_some() && h.is_some(){
            cfg.size = (w.unwrap(), h.unwrap());
        }
        if let Some(cell_size) = cfg.noise_params.cell_size{
            cfg.noise_params.cell_size = Some(cell_size);
        } else {
            cfg.noise_params.cell_size = Some(max(cfg.size.0, cfg.size.1))
        }
        return cfg;
    }

    fn load_from_file(fname: String, config_id: Option<String>) -> Self{
        let mut cfg = Config::defaults();
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");
        let docs = YamlLoader::load_from_str(contents.as_str()).expect("Cannot decode config file");

        for doc in docs {
            if let Some(cid) = config_id.as_ref() {
                let id: Option<&str> = doc["id"].as_str();
                if id.is_none() || !id.unwrap().eq(&cid.to_owned()){
                    continue;
                }
            }

            if !doc["kind"].as_str().expect("Settings formats must have a kind discriminator").eq("settings") {
                continue;
            }

            if let Some(v) = doc["id"].as_str() { cfg.id = v.to_string(); }
            if let Some(v) = doc["seed"].as_i64() { cfg.seed = v as u64; }
            if let Some(v) = doc["obstacles"].as_i64() { cfg.obstacles = v as usize; }
            if let Some(v) = doc["time_max"].as_i64() { cfg.time_max = v as usize; }
            if let Some(v) = doc["aux_path"].as_str() { cfg.aux_path = Some(v.to_string()); }
            if !doc["agents"].is_badvalue() {
                if let Some(v) = doc["agents"]["number"].as_i64() { cfg.agents.number = v as usize; }
                if let Some(v) = doc["agents"]["stop_probability"].as_f64() { cfg.agents.stop_probability = v; }
            }
            if !doc["noise"].is_badvalue() {
                cfg.noise_params.amplitude = doc["noise"]["amplitude"].as_f64();
                if let Some(v) = doc["noise"]["octaves"].as_i64() { cfg.noise_params.octaves = Some(v as usize); }
                cfg.noise_params.frequency = doc["noise"]["frequency"].as_f64();
                cfg.noise_params.persistence = doc["noise"]["persistence"].as_f64();
                cfg.noise_params.lacunarity = doc["noise"]["lacunarity"].as_f64();
                if let Some(v) = doc["noise"]["cell_size"].as_i64() { cfg.noise_params.cell_size = Some(v as usize); }
            }


            if !doc["size"]["width"].is_badvalue() && !doc["size"]["height"].is_badvalue() {
                cfg.size = (doc["size"]["width"].as_i64().unwrap() as usize, doc["size"]["height"].as_i64().unwrap() as usize);
            }
        }
        return cfg;
    }

    fn defaults() -> Self{
        return Config{
            id: "none".to_string(),
            seed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros() as u64,
            obstacles: 30,
            size: (10, 10),
            time_max: 100,
            aux_path: None,
            agents: AgentParams{
                number: 1,
                stop_probability: 0.0
            },
            noise_params: NoiseParams{
                amplitude: None,
                octaves: None,
                frequency: None,
                persistence: None,
                lacunarity: None,
                cell_size: None,
            }
        }
    }

}