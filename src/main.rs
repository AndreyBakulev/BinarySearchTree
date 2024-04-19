mod node;
mod tree;

use std::io;
use crate::node::Node;
use crate::tree::Tree;
fn main() {
    let mut tree = Tree::new();
    println!("Welcome To Andrey's Binary Search Tree!");
    let mut root_value = String::new();
    println!("Please enter a root node value!");
    io::stdin().read_line(&mut root_value).expect("Failed to read input");
    let root_value: i64 = root_value.trim().parse().expect("Invalid input");
    let mut node_values = String::new();
    println!("Please enter other node values!");
    io::stdin().read_line(&mut node_values).expect("Failed to read input");
    let node_values: Vec<&str> = node_values.split(",").collect();
    for i in 0.. node_values.len(){
        tree.insert(node_values[i].trim().parse::<i64>().unwrap());
    }
    tree.print_tree();
    tree.find(123);
}


/*TODO:
    write all methods with recursion and while true
    benchmark them to see which is faster
 */