pub trait Topology {
    type NodeId;
    type EdgeId;

    fn node_ids(&self) -> impl Iterator<Item = Self::NodeId> + '_;
    fn edge_ids(&self) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn node_edges(&self, id: Self::NodeId) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn endpoints(&self, id: Self::EdgeId) -> impl Iterator<Item = Self::NodeId> + '_;

    fn opposite(&self, eid: Self::EdgeId, nid: Self::NodeId) -> Option<Self::NodeId> {
        match self.endpoints(eid) {
            (a, b) if a == nid => Some(b),
            (a, b) if b == nid => Some(a),
            _ => None,
        }
    }
    fn node_count(&self) -> usize {
        self.node_ids().count()
    }

    fn edge_count(&self) -> usize {
        self.edge_ids().count()
    }
}
