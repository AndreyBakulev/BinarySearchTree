mod node;
mod tree;

use std::io;
use crate::tree::Tree;
use crate::node::Node;

fn main() {

    println!("Welcome To Andrey's Binary Search Tree!");
    let mut root_value = String::new();
    println!("Please enter a root node value!");
    io::stdin().read_line(&mut root_value).expect("Failed to read input");
    let root_value: i64 = root_value.trim().parse().expect("Invalid input");
    let mut tree = Tree::new(Node::new(root_value, None, None));
    let mut node_values = String::new();
    println!("Please enter other node values!");
    io::stdin().read_line(&mut node_values).expect("Failed to read input");
    let test1: Vec<&str> = node_values.split(",").collect();
    for i in 0.. test1.len(){
        tree.insert(test1[i].trim().parse::<i64>().unwrap());
    }
    tree.print_tree();
}
