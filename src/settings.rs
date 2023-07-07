struct MutationProbabilities {
    pub prob_mutation_weight: f64,
    pub prob_mutation_weight_perturbation: f64,
    pub prob_mutation_new_node: f64,
    pub prob_mutation_new_connection: f64,
    pub prob_inherit_on_fitter_genomre: f64,
    pub prob_inherit_disabled_gene: f64,

    pub prob_offsprint_crossover: f64,
    pub prob_mating_interspecies: f64,
}

impl Default for MutationProbabilities {
    fn default() -> Self {
        Self {
            prob_mutation_weight: 0.8,
            prob_mutation_weight_perturbation: 0.9,
            prob_mutation_new_node: 0.02,
            prob_mutation_new_connection: 0.05,
            prob_inherit_on_fitter_genomre: 0.5,
            prob_inherit_disabled_gene: 0.75,

            prob_offsprint_crossover: 0.75,
            prob_mating_interspecies: 0.001,
        }
    }
}

pub struct SimilarityCoeficients {
    pub coeficient_excess: f64,
    pub coeficient_disjoint: f64,
    pub coeficient_weight: f64,
}

impl Default for SimilarityCoeficients {
    fn default() -> Self {
        Self {
            coeficient_excess: 1.0,
            coeficient_disjoint: 1.0,
            coeficient_weight: 3.0,
        }
    }
}

pub struct NeatSettings {
    pub mutation_probabilities: MutationProbabilities,
    pub species_similarity_threshold: f64,
    pub normalized_gene_size: usize,
    pub similarity_coeficients: SimilarityCoeficients,
    pub size_specie_for_champion: usize,
    pub generation_for_stagnating_species: usize,
}

impl Default for NeatSettings {
    fn default() -> Self {
        Self {
            mutation_probabilities: Default::default(),
            species_similarity_threshold: 4.0,
            normalized_gene_size: 20,
            similarity_coeficients: Default::default(),
            size_specie_for_champion: 5,
            generation_for_stagnating_species: 15,
        }
    }
}
