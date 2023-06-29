use super::genome::GenomeId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SpecieId(pub usize);

pub struct Specie {
    pub id: SpecieId,
    pub genomes: Vec<GenomeId>,
    pub representative: GenomeId,
    pub fitness_sum: f64,
    pub fitness_max: f64,
    pub stagnant_generations: usize,
    pub bounds: usize,
    pub population: usize,
}
