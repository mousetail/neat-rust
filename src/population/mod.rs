use crate::settings::NeatSettings;

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

    fn nextGeneration<RNG: rand::Rng>(&mut self, rng: &mut RNG, settings: NeatSettings) {
        self.speciate(settings);
        self.compute_adjusted_fitness();
        self.sort_species();
        self.compute_stagnation(settings);
        self.compute_species_bounds();
        self.offspring_species(rng, settings);
        self.elect_representatives_for_species(rng);
        self.clean_species();
    }

    fn speciate(&mut self, settings: NeatSettings) {
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
                    if self.genomes[*index].similarity(genome)
                        > settings.species_similarity_threshold
                    {
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
        for genome in self.genomes.iter_mut() {
            genome.adjusted_fitness =
                genome.fitness / (self.species[genome.species.0].genomes.len() as f64);

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

    fn offspring_species<RNG: rand::Rng>(&mut self, rng: &mut RNG, settings: NeatSettings) {
        if self.species.iter().all(|i| i.genomes.is_empty()) {
            let bound = self.genomes.len() / 2;

            for (index, genome) in self.genomes.iter_mut().enumerate() {
                if index < bound {
                    *genome = Genome::new_random(rng, genome.id);
                } else {
                    genome.mutate(rng, settings);
                }
            }
            return;
        }

        for specie in self.species {
            if specie.genomes.is_empty() {
                continue;
            }

            let bound = specie.bounds;
            assert!(bound > 0);

            let skip_champions =
                (settings.size_specie_for_champion < specie.genomes.len()) as usize;
            for genome in specie.genomes.iter().skip(skip_champions) {
                self.genomes[genome.0].mutate(rng, settings);
            }

            for genome in specie.genomes.iter() {
                if rng.gen_bool(settings.mutation_probabilities.prob_offsprint_crossover) {
                    let (parent1, parent2) = if (rng
                        .gen_bool(settings.mutation_probabilities.prob_mating_interspecies))
                    {
                        // Finding a random non-empty specie and a genome inside that
                        let mut other_species = rng.gen_range(0..self.species.len());

                        while self.species[other_species].genomes.len() == 0
                            || self.species[other_species].bounds == 0
                        {
                            other_species = rng.gen_range(0..self.species.len())
                        }

                        let (genome_1, genome_2) = (
                            specie.genomes[rng.gen_range(0..specie.genomes.len())],
                            self.species[other_species].genomes
                                [rng.gen_range(0..self.species[other_species].genomes.len())],
                        );

                        if (self.genomes[genome_1.0].adjusted_fitness
                            < self.genomes[genome_2.0].adjusted_fitness)
                        {
                            (genome_1, genome_2)
                        } else {
                            (genome_2, genome_1)
                        }
                    } else {
                        let (genome_1, genome_2) = (
                            specie.genomes[rng.gen_range(0..specie.genomes.len())],
                            specie.genomes[rng.gen_range(0..specie.genomes.len())],
                        );

                        if (genome_1 < genome_2) {
                            (genome_1, genome_2)
                        } else {
                            (genome_2, genome_1)
                        }
                    };

                    self.genomes[genome.0] =
                        Genome::crossover(&self.genomes[parent1.0], &self.genomes[parent2.0]);
                }

                self.genomes[genome.0].mutate(rng, settings);
            }
        }
    }

    fn clean_species(&mut self) {
        self.species = self
            .species
            .into_iter()
            .filter(|i| i.genomes.len() > 0 || i.bounds == 0)
            .collect();

        for (index, specie) in self.species.iter_mut().enumerate() {
            specie.id = SpecieId(index);
            for genome in specie.genomes {
                self.genomes[genome.0].species = specie.id;
            }
        }
    }

    fn elect_representatives_for_species<RNG: rand::Rng>(&mut self, rng: &mut RNG) {
        for specie in self.species.iter_mut() {
            let num_to_keep = specie.bounds;

            if !specie.genomes.is_empty() && num_to_keep > 0 {
                specie.representative = specie.genomes[rng.gen_range(0..specie.genomes.len())];
            }
        }
    }

    fn compute_stagnation(&mut self, settings: NeatSettings) {
        for specie in self.species.iter_mut() {
            if (specie.genomes.is_empty()) {
                continue;
            }

            let current_max_fitness = self.compute_max_fitness_in_specie(specie);
            let historical_max_fitness = specie.fitness_max;
            if specie.stagnant_generations == 0 || historical_max_fitness < current_max_fitness {
                specie.fitness_max = current_max_fitness;
                specie.stagnant_generations = 1
            } else {
                specie.stagnant_generations += 1;

                if (settings.generation_for_stagnating_species > specie.stagnant_generations) {
                    specie.genomes.clear();
                }
            }
        }
    }

    fn compute_max_fitness_in_specie(&self, specie: &Specie) -> f64 {
        self.genomes[specie.genomes[0].0].adjusted_fitness
    }
}
