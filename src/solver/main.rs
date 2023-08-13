use common::field::field::{CustomField, Field, RandomField};
use common::noise::perlin::PerlinNoise;
use crate::args::Config;

mod args;

fn create_field_from_configs(cfg: &Config) -> Result<impl Field, &str>{
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

fn main() {
    let cfg = Config::load();

    // First of all create the field
    let field = create_field_from_configs(&cfg);

    println!("{:?}", cfg);
}