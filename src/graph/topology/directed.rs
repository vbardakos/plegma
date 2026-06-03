use super::{Degree, NodeTopology};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Kind {
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

impl Kind {
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

pub trait RelationalTopology: DirectedTopology {
    fn node_kind(&self, id: Self::NodeId) -> Kind {
        let parents = self.parents(id).filter(|&pid| pid != id);
        let children = self.children(id).filter(|&cid| cid != id);
        Kind::from_degrees(parents.collect(), children.collect())
    }

    fn is_orphan(&self, id: Self::NodeId) -> bool {
        self.node_kind(id) == Kind::Orphan
    }

    fn is_root(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), Kind::RelayRoot | Kind::ForkRoot)
    }

    fn is_leaf(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), Kind::RelayLeaf | Kind::JoinLeaf)
    }

    fn is_relay(&self, id: Self::NodeId) -> bool {
        matches!(
            self.node_kind(id),
            Kind::RelayLeaf | Kind::RelayRoot | Kind::Relay
        )
    }

    fn is_fork(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), Kind::Fork | Kind::ForkRoot)
    }

    fn is_join(&self, id: Self::NodeId) -> bool {
        matches!(self.node_kind(id), Kind::Join | Kind::JoinLeaf)
    }

    fn is_junction(&self, id: Self::NodeId) -> bool {
        self.node_kind(id) == Kind::Junction
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

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal directed-graph fixture: nodes + (parent, child) edges.
    struct Graph {
        nodes: Vec<u8>,
        edges: Vec<(u8, u8)>,
    }

    impl Graph {
        fn new(nodes: &[u8], edges: &[(u8, u8)]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                edges: edges.to_vec(),
            }
        }
    }

    impl NodeTopology for Graph {
        type NodeId = u8;
        fn node_ids(&self) -> impl Iterator<Item = u8> + '_ {
            self.nodes.iter().copied()
        }
    }

    impl DirectedTopology for Graph {
        fn parents(&self, id: u8) -> impl Iterator<Item = u8> + '_ {
            self.edges
                .iter()
                .filter_map(move |(p, c)| (*c == id).then_some(*p))
        }
        fn children(&self, id: u8) -> impl Iterator<Item = u8> + '_ {
            self.edges
                .iter()
                .filter_map(move |(p, c)| (*p == id).then_some(*c))
        }
    }

    impl RelationalTopology for Graph {}

    fn sorted<I: Iterator<Item = u8>>(iter: I) -> Vec<u8> {
        let mut v: Vec<u8> = iter.collect();
        v.sort_unstable();
        v
    }

    mod kind {
        use super::*;

        #[test]
        fn from_degrees_maps_each_combination_to_a_kind() {
            use Degree::*;
            use Kind::*;
            assert_eq!(Kind::from_degrees(Zero, Zero), Orphan);
            assert_eq!(Kind::from_degrees(Zero, One), RelayRoot);
            assert_eq!(Kind::from_degrees(Zero, Many), ForkRoot);
            assert_eq!(Kind::from_degrees(One, Zero), RelayLeaf);
            assert_eq!(Kind::from_degrees(One, One), Relay);
            assert_eq!(Kind::from_degrees(One, Many), Fork);
            assert_eq!(Kind::from_degrees(Many, Zero), JoinLeaf);
            assert_eq!(Kind::from_degrees(Many, One), Join);
            assert_eq!(Kind::from_degrees(Many, Many), Junction);
        }
    }

    mod directed_topology {
        use super::*;

        #[test]
        fn in_degrees_counts_parents() {
            // 1 -> 3, 2 -> 3
            let g = Graph::new(&[1, 2, 3], &[(1, 3), (2, 3)]);
            assert_eq!(g.in_degrees(3), 2);
            assert_eq!(g.in_degrees(1), 0);
        }

        #[test]
        fn out_degrees_counts_children() {
            // 1 -> 2, 1 -> 3
            let g = Graph::new(&[1, 2, 3], &[(1, 2), (1, 3)]);
            assert_eq!(g.out_degrees(1), 2);
            assert_eq!(g.out_degrees(2), 0);
        }

        #[test]
        fn has_self_loop_is_true_when_node_points_to_itself() {
            let g = Graph::new(&[1], &[(1, 1)]);
            assert!(g.has_self_loop(1));
        }

        #[test]
        fn has_self_loop_is_false_when_node_has_no_self_edge() {
            let g = Graph::new(&[1, 2], &[(1, 2)]);
            assert!(!g.has_self_loop(1));
            assert!(!g.has_self_loop(2));
        }
    }

    // One small graph per Kind, asserting node_kind on the target node.
    mod node_kind {
        use super::*;

        #[test]
        fn orphan_has_no_parents_and_no_children() {
            let g = Graph::new(&[1], &[]);
            assert_eq!(g.node_kind(1), Kind::Orphan);
        }

        #[test]
        fn relay_root_has_no_parents_and_one_child() {
            // 1 -> 2
            let g = Graph::new(&[1, 2], &[(1, 2)]);
            assert_eq!(g.node_kind(1), Kind::RelayRoot);
        }

        #[test]
        fn fork_root_has_no_parents_and_many_children() {
            // 1 -> 2, 1 -> 3
            let g = Graph::new(&[1, 2, 3], &[(1, 2), (1, 3)]);
            assert_eq!(g.node_kind(1), Kind::ForkRoot);
        }

        #[test]
        fn relay_leaf_has_one_parent_and_no_children() {
            // 1 -> 2
            let g = Graph::new(&[1, 2], &[(1, 2)]);
            assert_eq!(g.node_kind(2), Kind::RelayLeaf);
        }

        #[test]
        fn relay_has_one_parent_and_one_child() {
            // 1 -> 2 -> 3
            let g = Graph::new(&[1, 2, 3], &[(1, 2), (2, 3)]);
            assert_eq!(g.node_kind(2), Kind::Relay);
        }

        #[test]
        fn fork_has_one_parent_and_many_children() {
            // 1 -> 2; 2 -> 3, 2 -> 4
            let g = Graph::new(&[1, 2, 3, 4], &[(1, 2), (2, 3), (2, 4)]);
            assert_eq!(g.node_kind(2), Kind::Fork);
        }

        #[test]
        fn join_leaf_has_many_parents_and_no_children() {
            // 1 -> 3, 2 -> 3
            let g = Graph::new(&[1, 2, 3], &[(1, 3), (2, 3)]);
            assert_eq!(g.node_kind(3), Kind::JoinLeaf);
        }

        #[test]
        fn join_has_many_parents_and_one_child() {
            // 1 -> 3, 2 -> 3, 3 -> 4
            let g = Graph::new(&[1, 2, 3, 4], &[(1, 3), (2, 3), (3, 4)]);
            assert_eq!(g.node_kind(3), Kind::Join);
        }

        #[test]
        fn junction_has_many_parents_and_many_children() {
            // 1 -> 3, 2 -> 3; 3 -> 4, 3 -> 5
            let g = Graph::new(&[1, 2, 3, 4, 5], &[(1, 3), (2, 3), (3, 4), (3, 5)]);
            assert_eq!(g.node_kind(3), Kind::Junction);
        }

        #[test]
        fn self_loops_do_not_count_toward_node_kind() {
            // A self-loop alone must keep the node an Orphan.
            let g = Graph::new(&[1], &[(1, 1)]);
            assert_eq!(g.node_kind(1), Kind::Orphan);
        }
    }

    // Each iterator helper gets its own focused graph that exhibits the kinds it should
    // include and at least one kind it should exclude.
    mod relational_iterators {
        use super::*;

        #[test]
        fn orphans_yields_only_disconnected_nodes() {
            // 1 is orphan; 2 -> 3 has no orphans.
            let g = Graph::new(&[1, 2, 3], &[(2, 3)]);
            assert_eq!(sorted(g.orphans()), vec![1]);
        }

        #[test]
        fn roots_yields_relay_roots_and_fork_roots() {
            // 1 -> 2 (1 = RR); 3 -> 4, 3 -> 5 (3 = FR)
            let g = Graph::new(&[1, 2, 3, 4, 5], &[(1, 2), (3, 4), (3, 5)]);
            assert_eq!(sorted(g.roots()), vec![1, 3]);
        }

        #[test]
        fn leaves_yields_relay_leaves_and_join_leaves() {
            // 1 -> 2 (2 = RL); 3 -> 5, 4 -> 5 (5 = JL)
            let g = Graph::new(&[1, 2, 3, 4, 5], &[(1, 2), (3, 5), (4, 5)]);
            assert_eq!(sorted(g.leaves()), vec![2, 5]);
        }

        #[test]
        fn relays_yields_relay_relay_roots_and_relay_leaves() {
            // 1 -> 2 -> 3: 1 = RR, 2 = Relay, 3 = RL — all three are relays.
            let g = Graph::new(&[1, 2, 3], &[(1, 2), (2, 3)]);
            assert_eq!(sorted(g.relays()), vec![1, 2, 3]);
        }

        #[test]
        fn forks_yields_fork_and_fork_root() {
            // 1 -> 2, 1 -> 3 (1 = FR); 4 -> 5, 5 -> 6, 5 -> 7 (5 = Fork)
            let g = Graph::new(
                &[1, 2, 3, 4, 5, 6, 7],
                &[(1, 2), (1, 3), (4, 5), (5, 6), (5, 7)],
            );
            assert_eq!(sorted(g.forks()), vec![1, 5]);
        }

        #[test]
        fn joins_yields_join_and_join_leaf() {
            // 1 -> 3, 2 -> 3 (3 = JL); 4 -> 6, 5 -> 6, 6 -> 7 (6 = Join)
            let g = Graph::new(
                &[1, 2, 3, 4, 5, 6, 7],
                &[(1, 3), (2, 3), (4, 6), (5, 6), (6, 7)],
            );
            assert_eq!(sorted(g.joins()), vec![3, 6]);
        }

        #[test]
        fn junctions_yields_only_junctions() {
            // 1 -> 3, 2 -> 3, 3 -> 4, 3 -> 5 (3 = Junction)
            let g = Graph::new(&[1, 2, 3, 4, 5], &[(1, 3), (2, 3), (3, 4), (3, 5)]);
            assert_eq!(sorted(g.junctions()), vec![3]);
        }
    }
}
