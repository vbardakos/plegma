pub mod directed;

use super::topology::{EdgeTopology, NodeTopology};

pub struct RefNode<'g, G>
where
    G: NodeTopology,
{
    id: G::NodeId,
    graph: &'g G,
}

impl<'g, G> RefNode<'g, G>
where
    G: NodeTopology,
{
    fn new(id: G::NodeId, graph: &'g G) -> Self {
        Self { id, graph }
    }
    pub fn id(&self) -> G::NodeId {
        self.id
    }
}

impl<'g, G> RefNode<'g, G>
where
    G: EdgeTopology,
{
    pub fn edges(&self) -> impl Iterator<Item = G::EdgeId> + '_ {
        self.graph.node_edges(self.id)
    }
}
