use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use std::io;
use crate::tree::Tree;
mod node;
mod tree;
fn main() {
    let mut tree = Tree::new();
    println!("Welcome To Andrey's Binary Search Tree!");
    let mut root_value = String::new();
    println!("Please enter a root node value!");
    io::stdin().read_line(&mut root_value).expect("Failed to read input");
    let root_value: i64 = root_value.trim().parse().expect("Invalid input");
    tree.insert(root_value);
    let mut node_values = String::new();
    println!("Please enter other node values!");
    io::stdin().read_line(&mut node_values).expect("Failed to read input");
    let node_values: Vec<&str> = node_values.split(",").collect();
    for i in 0..node_values.len() {
        tree.insert(node_values[i].trim().parse::<i64>().unwrap());
    }
    let main_window = WindowDesc::new(move || tree.build_tree_widget())
        .title(LocalizedString::new("Binary Search Tree"))
        .window_size((400.0, 400.0));
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}