use std::fs;
use argparse::{ArgumentParser, StoreOption};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConfigTypes{
    Instance {
        #[serde(flatten)]
        config: Config
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
    pub id: String,
    pub seed: u64,
    pub greedy: bool,
    pub grid: GridConfig,
    pub aux_path: Option<String>,
    pub agents: AgentsConfig,
    pub time_max: usize,
    pub init: (usize, usize),
    pub goal: (usize, usize)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridConfig {
    pub width: usize,
    pub height: usize,
    pub obstacles: usize,
    pub noise: Option<NoiseConfig>,
    pub custom: Option<Vec<(usize, usize)>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoiseConfig {
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
    pub amplitude: f64,
    pub frequency: f64,
    pub cell_size: usize,
    pub val_limit: u32,
    pub cell_limit: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub paths: Vec<Vec<(usize, usize)>>
}

impl Config{
    pub fn load() -> Self{
        let mut fname: Option<String> = None;
        let mut conf_id: Option<String> = None;
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Instance Solver");

            ap.refer(&mut fname).add_option(&["-c", "--config"], StoreOption, "Config file name. If present configuration will be loaded from file INSTEAD of cmdline.");
            ap.refer(&mut conf_id).add_option(&["-i", "--config-id"], StoreOption, "Config ID. Allows to load one doc from the yaml file");
            ap.parse_args_or_exit();
        }

        let contents = fs::read_to_string(fname.expect("File not specified")).expect("Should have been able to read the file");
        let mut configs: Option<Config>;

        for document in serde_yaml::Deserializer::from_str(contents.as_str()) {
            match ConfigTypes::deserialize(document){
                Ok(e) => {
                    return match e {
                        ConfigTypes::Instance { config } => {
                            if let Some(wanted_id) = conf_id.as_ref() {
                                if !config.id.eq(wanted_id) {
                                    continue
                                }
                            }
                            config
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }

        panic!("Cannot read a valid config")
    }
}