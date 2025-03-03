use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

type NodePriorityQueue<'a, T> = BinaryHeap<NodeOrderHelper<'a, T>>;
#[derive(Debug)]
struct NodeOrderHelper<'a, T>(Node<'a, T>);

impl<'a, T> PartialEq<Self> for NodeOrderHelper<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.weight() == other.0.weight()
    }
}
impl<'a, T> Eq for NodeOrderHelper<'a, T> {}
impl<'a, T> PartialOrd for NodeOrderHelper<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.weight().partial_cmp(&other.0.weight())?.reverse())
    }
}
impl<'a, T> Ord for NodeOrderHelper<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .weight()
            .partial_cmp(&other.0.weight())
            .unwrap()
            .reverse()
    }
}
#[derive(Debug)]
struct LeafNode<'a, T> {
    symbol: &'a T,
    weight: usize,
}
#[derive(Debug)]
struct InternalNode<'a, T> {
    weight: usize,
    left_child: Node<'a, T>,
    right_child: Node<'a, T>,
}
#[derive(Debug)]
enum Node<'a, T> {
    Leaf(Box<LeafNode<'a, T>>),
    Internal(Box<InternalNode<'a, T>>),
}
impl<T> Node<'_, T> {
    fn weight(&self) -> usize {
        match self {
            Node::Leaf(leaf) => leaf.weight,
            Node::Internal(node) => node.weight,
        }
    }
}
fn merge_2_smallest_node<T>(tree: &mut NodePriorityQueue<T>) {
    // Unsure what's the proper name for this function
    let smallest_node = tree.pop().expect("should not be empty").0;
    let second_smallest_node = tree.pop().expect("len should be >=2").0;
    tree.push(NodeOrderHelper(Node::Internal(Box::new(InternalNode {
        weight: smallest_node.weight() + second_smallest_node.weight(),
        left_child: second_smallest_node,
        right_child: smallest_node,
    }))));
}
fn create_tree<'a, T>(data: &'a [T]) -> (NodePriorityQueue<'a, T>, usize)
where
    T: Eq,
    T: Hash,
{
    let mut probability: HashMap<&T, usize> = HashMap::new();
    for i in data {
        probability.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut tree: NodePriorityQueue<'a, T> = BinaryHeap::with_capacity(probability.len());
    for (symbol, occurrence) in probability.iter() {
        tree.push(NodeOrderHelper(Node::Leaf(Box::new(LeafNode {
            symbol: *symbol,
            weight: *occurrence,
        }))))
    }
    let unique_symbol_len= tree.len();
    while tree.len() > 1 {
        merge_2_smallest_node(&mut tree);
    }
    (tree, unique_symbol_len)
}
fn generate_lookup<'a, T>(root_node: Node<'a, T>, depth: usize) -> HashMap<&'a T, usize>
where
    T: Eq,
    T: Hash,
{
    let mut lookup: HashMap<&'a T, usize> = HashMap::with_capacity(depth);
    let mut walk_queue = VecDeque::with_capacity(2);
    walk_queue.push_back((root_node, 0));
    while let Some((node, current)) = walk_queue.pop_front() {
        match node {
            Node::Leaf(leaf) => {
                lookup.insert(leaf.symbol, current);
            }
            Node::Internal(node) => {
                if matches!(node.left_child, Node::Internal(_)) {
                    walk_queue.push_back((node.left_child, (current << 1) + 1));
                    walk_queue.push_back((node.right_child, current << 1));
                } else {
                    walk_queue.push_back((node.right_child, (current << 1) + 1));
                    walk_queue.push_back((node.left_child, current << 1));
                }
            }
        }
    }

    lookup
}
fn main() {
    //let string = "Hello, world! World test!";
    let string = "A".repeat(40) + &*"B".repeat(35) + &*"C".repeat(20) + &*"D".repeat(5);
    //let string="this is an example of a huffman tree";
    println!("{:?}", string);
    let data = string.chars().collect::<Vec<char>>();
    let (mut tree, _) = create_tree(&data);
    let tree = tree.pop().unwrap().0;
    println!("{:#?}", tree);
    let lookup = generate_lookup(tree, depth);
    println!("{:#?}", lookup);
}
