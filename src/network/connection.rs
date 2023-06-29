use super::node::NodeId;

pub struct Connection {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64,
    pub enabled: bool,
}
