pub mod directed;
pub mod undirected;

pub trait NodeTopology {
    type NodeId: Copy + Eq;

    fn node_ids(&self) -> impl Iterator<Item = Self::NodeId> + '_;
    fn node_count(&self) -> usize {
        self.node_ids().count()
    }
}

pub trait EdgeTopology: NodeTopology {
    type EdgeId;

    fn edge_ids(&self) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn node_edges(&self, id: Self::NodeId) -> impl Iterator<Item = Self::EdgeId> + '_;
    fn endpoints(&self, id: Self::EdgeId) -> Option<(Self::NodeId, Self::NodeId)>;

    fn edge_count(&self) -> usize {
        self.edge_ids().count()
    }

    fn opposite(&self, eid: Self::EdgeId, nid: Self::NodeId) -> Option<Self::NodeId> {
        match self.endpoints(eid)? {
            (a, b) if a == nid => Some(b),
            (a, b) if b == nid => Some(a),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Degree {
    Zero,
    One,
    Many,
}

impl<T> FromIterator<T> for Degree {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match (iter.next(), iter.next()) {
            (Some(_), None) => Self::One,
            (Some(_), Some(_)) => Self::Many,
            _ => Self::Zero,
        }
    }
}

impl From<usize> for Degree {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            _ => Self::Many,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod degree {
        use super::*;

        #[test]
        fn from_iter_empty_is_zero() {
            let d: Degree = std::iter::empty::<()>().collect();
            assert_eq!(d, Degree::Zero);
        }

        #[test]
        fn from_iter_single_item_is_one() {
            let d: Degree = std::iter::once(()).collect();
            assert_eq!(d, Degree::One);
        }

        #[test]
        fn from_iter_two_items_is_many() {
            let d: Degree = [(), ()].into_iter().collect();
            assert_eq!(d, Degree::Many);
        }

        #[test]
        fn from_iter_stops_after_second_item() {
            let mut yielded = 0;
            let iter = std::iter::from_fn(|| {
                yielded += 1;
                assert!(yielded <= 2, "Degree must not pull more than two items");
                Some(())
            });
            let d: Degree = iter.collect();
            assert_eq!(d, Degree::Many);
        }

        #[test]
        fn from_usize_zero_is_zero() {
            assert_eq!(Degree::from(0_usize), Degree::Zero);
        }

        #[test]
        fn from_usize_one_is_one() {
            assert_eq!(Degree::from(1_usize), Degree::One);
        }

        #[test]
        fn from_usize_two_or_more_is_many() {
            for n in [2_usize, 3, 17, usize::MAX] {
                assert_eq!(Degree::from(n), Degree::Many, "n = {n}");
            }
        }

        #[test]
        fn ordering_is_zero_lt_one_lt_many() {
            assert!(Degree::Zero < Degree::One);
            assert!(Degree::One < Degree::Many);
        }
    }

    mod edge_topology {
        use super::*;

        // Minimal topology fixture: nodes are u8, edges are positions in a (u8, u8) vec.
        struct Graph {
            nodes: Vec<u8>,
            edges: Vec<(u8, u8)>,
        }

        impl NodeTopology for Graph {
            type NodeId = u8;
            fn node_ids(&self) -> impl Iterator<Item = u8> + '_ {
                self.nodes.iter().copied()
            }
        }

        impl EdgeTopology for Graph {
            type EdgeId = usize;
            fn edge_ids(&self) -> impl Iterator<Item = usize> + '_ {
                0..self.edges.len()
            }
            fn node_edges(&self, id: u8) -> impl Iterator<Item = usize> + '_ {
                self.edges
                    .iter()
                    .enumerate()
                    .filter_map(move |(i, (a, b))| (*a == id || *b == id).then_some(i))
            }
            fn endpoints(&self, id: usize) -> Option<(u8, u8)> {
                self.edges.get(id).copied()
            }
        }

        #[test]
        fn node_count_matches_node_ids_count() {
            let g = Graph { nodes: vec![1, 2, 3], edges: vec![] };
            assert_eq!(g.node_count(), 3);
        }

        #[test]
        fn edge_count_matches_edge_ids_count() {
            let g = Graph { nodes: vec![1, 2], edges: vec![(1, 2), (2, 1)] };
            assert_eq!(g.edge_count(), 2);
        }

        #[test]
        fn opposite_returns_the_other_endpoint_from_either_side() {
            let g = Graph { nodes: vec![1, 2], edges: vec![(1, 2)] };
            assert_eq!(g.opposite(0, 1), Some(2));
            assert_eq!(g.opposite(0, 2), Some(1));
        }

        #[test]
        fn opposite_on_a_self_loop_returns_the_same_node() {
            let g = Graph { nodes: vec![7], edges: vec![(7, 7)] };
            assert_eq!(g.opposite(0, 7), Some(7));
        }

        #[test]
        fn opposite_returns_none_when_node_is_not_an_endpoint() {
            let g = Graph { nodes: vec![1, 2], edges: vec![(1, 2)] };
            assert_eq!(g.opposite(0, 99), None);
        }

        #[test]
        fn opposite_returns_none_when_edge_id_is_unknown() {
            let g = Graph { nodes: vec![1, 2], edges: vec![] };
            assert_eq!(g.opposite(0, 1), None);
        }
    }
}
