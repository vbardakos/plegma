pub trait Topology {
    type NodeId;
    type EdgeId;

    fn node_edges(&self, id: Self::NodeId) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn endpoints(&self, id: Self::EdgeId) -> impl Iterator<Item = Self::NodeId> + '_;
}

