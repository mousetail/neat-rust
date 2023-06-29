use self::{
    genome::{Genome, GenomeId},
    specie::{Specie, SpecieId},
};

pub mod genome;
pub mod specie;

pub struct Population<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, const BIAS: bool> {
    genomes: Vec<Genome<INPUT_SIZE, OUTPUT_SIZE, BIAS>>,
    species: Vec<Specie>,
}

impl<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, const BIAS: bool>
    Population<INPUT_SIZE, OUTPUT_SIZE, BIAS>
{
    pub fn new<RNG: rand::Rng>(population_size: usize, rng: &mut RNG) -> Self {
        let genomes: Vec<Genome<INPUT_SIZE, OUTPUT_SIZE, BIAS>> = (0..population_size)
            .map(|i| Genome::new_random(rng, GenomeId(i)))
            .collect();

        Population {
            genomes,
            species: vec![],
        }
    }

    fn nextGeneration<RNG: rand::Rng>(&mut self, rng: &mut RNG) {
        self.speciate();
        self.compute_adjusted_fitness();
        self.sort_species();
        self.compute_stagnation();
        self.compute_species_bounds();
        self.offspring_species(rng);
        self.select_representative_for_species(rng);
        self.clean_species();
    }

    fn speciate(&mut self) {
        for specie in self.species.iter_mut() {
            specie.fitness_sum = 0.0;
            specie.genomes.clear();
        }

        let mut representative_indexes: Vec<Option<usize>> = self
            .species
            .iter()
            .map(|specie| {
                (0..)
                    .zip(self.genomes)
                    .find(|j| j.1.id == specie.representative)
                    .and_then(|i| Some(i.0))
            })
            .collect();

        let mut representative_specie_ids = Vec::<SpecieId>::new();
        for genome in self.genomes.iter() {
            let mut specie: Option<usize> = None;
            for (specie_index, representative_index) in representative_indexes.iter().enumerate() {
                if let Some(index) = representative_index {
                    if self.genomes[*index].similarity(genome) > 0.95 {
                        specie = Some(specie_index);
                        break;
                    }
                }
            }

            if let None = specie {
                let new_specie_id = self.species.len();

                representative_indexes.push(Some(new_specie_id));
                self.species.push(Specie {
                    id: SpecieId(new_specie_id),
                    genomes: vec![],
                    representative: genome.id,
                    fitness_max: 0.0,
                    fitness_sum: 0.0,
                    bounds: 0,
                    stagnant_generations: 0,
                    population: 1,
                });

                representative_specie_ids.push(SpecieId(new_specie_id))
            } else if let Some(specie_index) = specie {
                representative_specie_ids.push(self.species[specie_index].id)
            }
        }

        for (specie, genome) in representative_specie_ids
            .into_iter()
            .zip(self.genomes.iter_mut())
        {
            genome.species = specie;
            self.species[specie.0].genomes.push(genome.id);
        }
    }

    fn compute_adjusted_fitness(&mut self) {
        for specie in self.species.iter_mut() {
            specie.population = 0;
        }

        for genome in self.genomes {
            self.species[genome.species.0].population += 1;
        }

        for genome in self.genomes.iter_mut() {
            genome.adjusted_fitness =
                genome.fitness / (self.species[genome.species.0].population as f64);

            self.species[genome.species.0].fitness_sum += genome.adjusted_fitness;
        }
    }

    fn sort_species(&mut self) {
        for specie in self.species.iter_mut() {
            specie.genomes.sort_unstable_by(|genome_1, genome_2| {
                self.genomes[genome_1.0]
                    .adjusted_fitness
                    .total_cmp(&self.genomes[genome_2.0].adjusted_fitness)
            })
        }
    }

    fn compute_species_bounds(&mut self) {
        let sum_fitness = self
            .species
            .iter()
            .map(|specie| specie.fitness_sum)
            .sum::<f64>();

        for specie in self.species.iter_mut() {
            specie.bounds =
                (specie.fitness_sum / sum_fitness * specie.genomes.len() as f64).ceil() as usize;
        }
    }

    fn offspring_species(&mut self) {}
}
