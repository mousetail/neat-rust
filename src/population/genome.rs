use rand;

use crate::network::{
    connection::Connection,
    node::{Node, NodeId},
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
        1.0
    }
}
