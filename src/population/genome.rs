use rand;

use crate::{
    network::{
        connection::Connection,
        node::{Node, NodeId},
    },
    settings::NeatSettings,
};

use super::specie::SpecieId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GenomeId(pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LayerId(pub usize);

impl LayerId {
    const MAX: LayerId = LayerId(usize::MAX / 2);
}

pub struct Genome<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, const BIAS: bool> {
    pub id: GenomeId,
    pub connections: Vec<Connection>,
    pub layers: Vec<LayerId>,
    pub nodes: Vec<Vec<Node>>,

    pub fitness: f64,
    pub adjusted_fitness: f64,
    pub species: SpecieId,
}

impl<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize, const BIAS: bool>
    Genome<INPUT_SIZE, OUTPUT_SIZE, BIAS>
{
    pub fn new_random<RNG: rand::Rng>(
        rng: &mut RNG,
        id: GenomeId,
    ) -> Genome<INPUT_SIZE, OUTPUT_SIZE, BIAS> {
        let total_connetions = (INPUT_SIZE + BIAS as usize) * OUTPUT_SIZE;
        let last_node_index = NodeId(INPUT_SIZE + OUTPUT_SIZE + 1);

        let first_layer = LayerId(0);
        let last_layer = LayerId::MAX;

        let layers = vec![first_layer, last_layer];

        let nodes: Vec<Vec<Node>> = vec![
            (0..INPUT_SIZE + BIAS as usize)
                .map(|i| Node {
                    id: NodeId(i),
                    value: 0.0,
                    layer: first_layer,
                })
                .collect(),
            (0..OUTPUT_SIZE)
                .map(|i| Node {
                    id: NodeId(i + INPUT_SIZE + BIAS as usize),
                    value: 0.0,
                    layer: last_layer,
                })
                .collect(),
        ];

        let connections = nodes[0]
            .iter()
            .flat_map(|i| {
                nodes[1].iter().map(|j| Connection {
                    from: i.id,
                    to: j.id,
                    weight: rng.sample(rand::distributions::Uniform::new(0.0, 1.0)),
                    enabled: true,
                })
            })
            .collect();

        Genome {
            id,
            connections,
            layers,
            nodes,
            fitness: 0.0,
            adjusted_fitness: 0.0,
            species: SpecieId(0),
        }
    }

    pub fn similarity(&self, other: &Self) -> f64 {
        let mut num_matching: usize = 0;
        let mut num_excess: usize = 0;
        let mut num_disjoint: usize = 0;
        let mut weight_difference: f64 = 0.0;

        for (i, j) in self.connections.iter().zip(other.connections.iter()) {
            if i.get_hash() == j.get_hash() {
                num_matching += 1;
            } else {
                num_disjoint += 1;
            }
        }

        weight_difference
    }

    pub fn mutate<RNG: rand::Rng>(&mut self, rng: RNG, settings: NeatSettings) {
        todo!();
    }

    fn get_gene_by_hash(&self, hash: usize) -> Option<Connection> {
        todo!();
    }

    pub fn crossover<RNG: rand::Rng>(
        parent_1: &Self,
        parent_2: &Self,
        id: GenomeId,
        rng: &mut RNG,
        settings: NeatSettings,
    ) -> Genome<INPUT_SIZE, OUTPUT_SIZE, BIAS> {
        let mut connections = Vec::with_capacity(parent_1.connections.len());

        for gene in parent_1.connections {
            let connection_hash = gene.get_hash();

            let mut new_gene = gene;
            if let Some(gene_2) = parent_2.get_gene_by_hash(connection_hash) {
                new_gene = gene_2;
            }

            connections.push(new_gene);
        }

        let out = Genome {
            id,
            connections,
            species: parent_1.species,
            layers: parent_1.layers,
            nodes: parent_1.nodes,
            fitness: 0.0,
            adjusted_fitness: 0.0,
        };

        assert!(out.validate());

        out
    }
}
