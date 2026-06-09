use crate::graph::topology::directed::{DirectedTopology, Kind, RelationalTopology};

pub trait NodeIdVisitor<G: RelationalTopology> {
    fn visit(&mut self, graph: &G) {
        use Kind::*;

        for id in graph.node_ids() {
            if graph.has_self_loop(id) {
                self.visit_self_loop(id);
            }

            match graph.node_kind(id) {
                Orphan => self.visit_orphan(id),
                RelayRoot => self.visit_relay_root(id),
                ForkRoot => self.visit_fork_root(id),
                RelayLeaf => self.visit_relay_leaf(id),
                Relay => self.visit_inner_relay(id),
                Fork => self.visit_inner_fork(id),
                JoinLeaf => self.visit_join_leaf(id),
                Join => self.visit_join(id),
                Junction => self.visit_inner_junction(id),
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_node(&mut self, id: G::NodeId) {}

    fn visit_orphan(&mut self, id: G::NodeId) {
        self.visit_node(id)
    }

    fn visit_root(&mut self, id: G::NodeId) {
        self.visit_node(id)
    }

    fn visit_relay_root(&mut self, id: G::NodeId) {
        self.visit_relay(id);
        self.visit_root(id)
    }

    fn visit_fork_root(&mut self, id: G::NodeId) {
        self.visit_fork(id);
        self.visit_root(id)
    }

    fn visit_leaf(&mut self, id: G::NodeId) {
        self.visit_node(id)
    }

    fn visit_relay_leaf(&mut self, id: G::NodeId) {
        self.visit_relay(id);
        self.visit_leaf(id)
    }

    fn visit_join_leaf(&mut self, id: G::NodeId) {
        self.visit_join(id);
        self.visit_leaf(id)
    }

    fn visit_inner(&mut self, id: G::NodeId) {
        self.visit_node(id);
    }

    fn visit_inner_join(&mut self, id: G::NodeId) {
        self.visit_join(id);
        self.visit_inner(id);
    }

    fn visit_inner_fork(&mut self, id: G::NodeId) {
        self.visit_fork(id);
        self.visit_inner(id);
    }

    fn visit_inner_relay(&mut self, id: G::NodeId) {
        self.visit_relay(id);
        self.visit_inner(id);
    }

    fn visit_inner_junction(&mut self, id: G::NodeId) {
        self.visit_inner(id);
    }

    #[allow(unused_variables)]
    fn visit_relay(&mut self, id: G::NodeId) {}

    #[allow(unused_variables)]
    fn visit_fork(&mut self, id: G::NodeId) {}

    #[allow(unused_variables)]
    fn visit_join(&mut self, id: G::NodeId) {}

    #[allow(unused_variables)]
    fn visit_self_loop(&mut self, id: G::NodeId) {}
}
