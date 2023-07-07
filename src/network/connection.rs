use super::node::NodeId;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Connection {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64,
    pub enabled: bool,
}

impl Connection {
    pub fn get_hash(&self) -> usize {
        todo!();
    }
}
