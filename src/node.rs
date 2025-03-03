use crate::code::HuffmanCode;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
pub(crate) struct NodeOrderHelper<'a, T>(Node<'a, T>);
impl<'a, T> NodeOrderHelper<'a, T> {
    pub(crate) fn new(node: Node<'a, T>) -> Self {
        NodeOrderHelper(node)
    }
    pub(crate) fn move_root(self) -> Node<'a, T> {
        self.0
    }
}
impl<'a, T> PartialEq<Self> for NodeOrderHelper<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.weight() == other.0.weight()
    }
}

impl<'a, T> Eq for NodeOrderHelper<'a, T> {}

impl<'a, T> PartialOrd for NodeOrderHelper<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.weight().cmp(&other.0.weight()).reverse())
    }
}

impl<'a, T> Ord for NodeOrderHelper<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.weight().cmp(&other.0.weight()).reverse()
    }
}
#[derive(Debug)]
pub(crate) struct LeafNode<'a, T> {
    pub(crate) symbol: &'a T,
    pub(crate) weight: usize,
}

#[derive(Debug)]
pub(crate) struct InternalNode<'a, T> {
    pub(crate) weight: usize,
    pub(crate) left_child: Node<'a, T>,
    pub(crate) right_child: Node<'a, T>,
}

#[derive(Debug)]
pub(crate) enum Node<'a, T> {
    Leaf(Box<LeafNode<'a, T>>),
    Internal(Box<InternalNode<'a, T>>),
}

impl<T> Node<'_, T> {
    pub(crate) fn weight(&self) -> usize {
        match self {
            Node::Leaf(leaf) => leaf.weight,
            Node::Internal(node) => node.weight,
        }
    }
}

pub(crate) fn generate_lookup<'a, T>(
    root_node: Node<'a, T>,
    symbol_count: usize,
) -> HashMap<&'a T, HuffmanCode>
where
    T: Eq + Hash,
{
    let mut lookup: HashMap<&'a T, HuffmanCode> = HashMap::with_capacity(symbol_count);
    let mut walk_queue = VecDeque::with_capacity(2);
    walk_queue.push_back((root_node, HuffmanCode::new()));
    while let Some((node, current)) = walk_queue.pop_front() {
        match node {
            Node::Leaf(leaf) => {
                lookup.insert(leaf.symbol, current);
            }
            Node::Internal(node) => {
                walk_queue.push_back((node.right_child, current.right()));
                walk_queue.push_back((node.left_child, current.left()));
            }
        }
    }
    lookup
}
