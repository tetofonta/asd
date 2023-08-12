pub mod perlin;

#[cfg(test)]
mod perlin_test{
    use crate::noise::perlin::PerlinNoise;

    #[test]
    fn one_cell() {
        let noise = PerlinNoise::new(Some(42), Some(3), None, None, None, None, Some(5));
        for i in 0..50{
            for j in 0..50{
                if noise.gen_normalized(i, j) > 0.6{
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}