use crate::population::genome::LayerId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NodeId(pub usize);

pub struct Node {
    pub id: NodeId,
    pub value: f64,
    pub layer: LayerId,
}
