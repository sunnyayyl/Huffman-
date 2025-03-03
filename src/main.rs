mod code;
mod node;
mod tree;

fn main() {
    //let string = "Hello, world! World test!";
    let string = "A".repeat(40) + &*"B".repeat(35) + &*"C".repeat(20) + &*"D".repeat(5);
    //let string = "this is an example of a huffman tree";
    println!("{:?}", string);
    let characters = string.chars().collect::<Vec<char>>();
    let (mut tree, count) = tree::create_tree(&characters);
    let tree = tree.pop().unwrap().move_root();
    println!("{:#?}", tree);
    let lookup = node::generate_lookup(tree, count);
    for (k, v) in lookup.iter() {
        println!("'{}': {}", k, v);
    }
}
