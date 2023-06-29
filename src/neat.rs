use rand;

use crate::{network::Network, population::Population};

pub struct Neat<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, RNG: rand::Rng> {
    rng: RNG,
    population: Population<INPUT_SIZE, OUTPUT_SIZE, true>,
    //networks: Vec<Network>,
    generation_number: usize,
}

impl<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, RNG: rand::Rng>
    Neat<INPUT_SIZE, OUTPUT_SIZE, RNG>
{
    fn new(population_size: usize, mut rng: RNG) -> Self {
        let population = Population::new(population_size, &mut rng);

        Neat {
            rng: rng,
            population: population,
            generation_number: 0,
        }
    }
}
