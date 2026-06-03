use super::RefNode;
use crate::graph::topology::directed::{DirectedTopology, Kind, RelationalTopology};

impl<'g, G: DirectedTopology> RefNode<'g, G, G::NodeId> {
    pub fn parents(&self) -> impl Iterator<Item = RefNode<'g, G, G::NodeId>> + '_ {
        self.graph
            .parents(self.id)
            .map(|id| RefNode::new(id, self.graph))
    }

    pub fn children(&self) -> impl Iterator<Item = RefNode<'g, G, G::NodeId>> + '_ {
        self.graph
            .children(self.id)
            .map(|id| RefNode::new(id, self.graph))
    }

    pub fn has_self_loop(&self) -> bool {
        self.graph.has_self_loop(self.id)
    }
}

impl<'g, G: RelationalTopology> RefNode<'g, G, G::NodeId> {
    pub fn kind(&self) -> Kind {
        self.graph.node_kind(self.id)
    }

    pub fn is_orphan(&self) -> bool {
        self.graph.is_orphan(self.id)
    }

    pub fn is_root(&self) -> bool {
        self.graph.is_root(self.id)
    }

    pub fn is_fork(&self) -> bool {
        self.graph.is_fork(self.id)
    }

    pub fn is_leaf(&self) -> bool {
        self.graph.is_leaf(self.id)
    }

    pub fn is_relay(&self) -> bool {
        self.graph.is_relay(self.id)
    }

    pub fn is_junction(&self) -> bool {
        self.graph.is_junction(self.id)
    }

    pub fn is_join(&self) -> bool {
        self.graph.is_join(self.id)
    }
}
