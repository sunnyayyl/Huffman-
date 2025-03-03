use crate::node::{InternalNode, LeafNode, Node, NodeOrderHelper};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

fn merge_2_smallest_node<T>(tree: &mut NodePriorityQueue<T>) {
    // Unsure what's the proper name for this function
    let smallest_node = tree.pop().expect("should not be empty").move_root();
    let second_smallest_node = tree.pop().expect("len should be >=2").move_root();
    tree.push(NodeOrderHelper::new(Node::Internal(Box::new(
        InternalNode {
            weight: smallest_node.weight() + second_smallest_node.weight(),
            left_child: smallest_node,
            right_child: second_smallest_node,
        },
    ))));
}

pub(crate) fn create_tree<'a, T>(data: &'a [T]) -> (NodePriorityQueue<'a, T>, usize)
where
    T: Eq + Hash,
{
    let mut probability: HashMap<&T, usize> = HashMap::new();
    for i in data {
        probability.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut tree: NodePriorityQueue<'a, T> = BinaryHeap::with_capacity(probability.len());
    for (symbol, occurrence) in probability.iter() {
        tree.push(NodeOrderHelper::new(Node::Leaf(Box::new(LeafNode {
            symbol: *symbol,
            weight: *occurrence,
        }))))
    }
    let unique_symbol_count = tree.len();
    while tree.len() > 1 {
        merge_2_smallest_node(&mut tree);
    }
    (tree, unique_symbol_count)
}

pub type NodePriorityQueue<'a, T> = BinaryHeap<NodeOrderHelper<'a, T>>;
