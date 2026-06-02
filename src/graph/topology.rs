pub trait NodeTopology {
    type NodeId;

    fn node_ids(&self) -> impl Iterator<Item = Self::NodeId> + '_;
    fn node_edges(&self, id: Self::NodeId) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn node_count(&self) -> usize {
        self.node_ids().count()
    }

    fn edge_count(&self) -> usize {
        self.edge_ids().count()
    }
}

pub trait EdgeTopology: NodeTopology {
    type EdgeId;

    fn edge_ids(&self) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn endpoints(&self, id: Self::EdgeId) -> impl Iterator<Item = Self::NodeId> + '_;

    fn opposite(&self, eid: Self::EdgeId, nid: Self::NodeId) -> Option<Self::NodeId> {
        match self.endpoints(eid) {
            (a, b) if a == nid => Some(b),
            (a, b) if b == nid => Some(a),
            _ => None,
        }
    }
}

pub trait DirectedTopology: NodeTopology {
    fn parents(&self, id: Self::NodeId) -> impl Iterator<Item = Self::NodeId> + '_;
    fn children(&self, id: Self::NodeId) -> impl Iterator<Item = Self::NodeId> + '_;

    fn in_degrees(&self, id: Self::NodeId) -> usize {
        self.parents(id).count()
    }

    fn out_degrees(&self, id: Self::NodeId) -> usize {
        self.children(id).count()
    }

    fn has_self_loop(&self, id: Self::NodeId) -> bool {
        self.parents(id).any(|pid| pid == id)
    }
}

pub enum Degree {
    Zero,
    One,
    Many,
}

pub enum LocalKind {
    Orphan,
    RelayRoot,
    ForkRoot,
    RelayLeaf,
    Relay,
    Fork,
    JoinLeaf,
    Join,
    Junction,
}

impl<NodeId> FromIterator<NodeId> for Degree {
    fn from_iter<T: IntoIterator<Item = NodeId>>(iter: T) -> Self {
        let mut values = iter.into_iter();
        match (values.next(), values.next()) {
            (Some(_), None) => Self::One,
            (Some(_), Some(_)) => Self::Many,
            _ => Self::Zero,
        }
    }
}

impl LocalKind {
    fn from_degrees(parents: Degree, children: Degree) -> Self {
        use Degree::*;
        match (parents, children) {
            (Zero, Zero) => Self::Orphan,
            (Zero, One) => Self::RelayRoot,
            (Zero, Many) => Self::ForkRoot,
            (One, Zero) => Self::RelayLeaf,
            (One, One) => Self::Relay,
            (One, Many) => Self::Fork,
            (Many, Zero) => Self::JoinLeaf,
            (Many, One) => Self::Join,
            (Many, Many) => Self::Junction,
        }
    }
}

pub trait RelationalTopology: DirectedTopology {
    fn node_kind(&self, id: Self::NodeId) -> LocalKind {
        let parents = self.parents(id).filter(|pid| pid != id);
        let children = self.children(id).filter(|cid| cid != id);
        LocalKind::from_degrees(Degree::from_iter(parents), Degree::from_iter(children))
    }

    fn is_orphan(&self, id: Self::NodeId) -> bool {
        self.node_kind(id) == LocalKind::Orphan
    }

    fn is_root(&self, id: Self::NodeId) -> bool {
        matches!(
            self.node_kind(id),
            LocalKind::RelayRoot | LocalKind::ForkRoot
        )
    }

    fn is_leaf(&self, id: Self::NodeId) -> bool {
        matches!(
            self.node_kind(id),
            LocalKind::RelayLeaf | LocalKind::JoinLeaf
        )
    }

    fn is_relay(&self, id: Self::NodeId) -> bool {
        matches!(
            self.node_kind(id),
            LocalKind::RelayLeaf | LocalKind::RelayRoot | LocalKind::Relay
        )
    }

    fn is_fork(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), LocalKind::Fork | LocalKind::ForkRoot)
    }

    fn is_join(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), LocalKind::Join | LocalKind::JoinLeaf)
    }

    fn is_junction(&self, id: Self::NodeId) -> bool {
        self.node_kind(id) == LocalKind::Junction
    }

    fn orphans(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_orphan(id))
    }

    fn roots(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_root(id))
    }

    fn leaves(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_leaf(id))
    }

    fn relays(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_relay(id))
    }

    fn forks(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_fork(id))
    }

    fn joins(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_join(id))
    }

    fn junctions(&self) -> impl Iterator<Item = Self::NodeId> + '_ {
        self.node_ids().filter(|&id| self.is_junction(id))
    }
}
