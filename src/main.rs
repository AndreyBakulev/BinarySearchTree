mod node;
mod tree;
use crate::tree::Tree;
use crate::node::Node;

fn main() {
    println!("Hello, world!");
    let tree = Tree::new(Node::new(50,None,None));
}
