use crate::node::Node;

pub(crate) struct Tree {
    root_node: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root_node: None,
        }
    }
    pub fn insert(&mut self, value: i64) {
        let new_node = Box::new(Node::new(value, None, None));
        let mut current_node = match self.root_node.as_mut() {
            Some(node) => node,
            None => {
                self.root_node = Some(Box::new(*new_node));
                return;
            }
        };
        loop {
            if value < current_node.value {
                if let Some(ref mut left_node) = current_node.left {
                    current_node = left_node;
                } else {
                    current_node.left = Some(new_node);
                    break;
                }
            } else {
                if let Some(ref mut right_node) = current_node.right {
                    current_node = right_node;
                } else {
                    current_node.right = Some(new_node);
                    break;
                }
            }
        }
    }

    pub fn find(&mut self, value: i64) {
        let mut current_node = self.root_node.as_mut().unwrap();
        loop {
            if value < current_node.value {
                if let Some(ref mut left_node) = current_node.left {
                    if left_node.value == value {
                        println!("Found Node as left node of {}", current_node.value);
                        break;
                    }
                    current_node = left_node;
                } else {
                    println!("Node not found in tree!");
                    break;
                }
            } else {
                if let Some(ref mut right_node) = current_node.right {
                    if right_node.value == value {
                        println!("Found Node as right node of {}", current_node.value);
                        break;
                    }
                    current_node = right_node;
                } else {
                    println!("Node not found in tree!");
                    break;
                }
            }
        }
    }

    pub fn print_tree(&self) {
        print_tree_rec(Box::new(&self).as_mut().root_node);
    }
}

fn print_tree_rec(node: Option<Box<Node>>) {
    match node {
        Some(node) => {
            print_tree_rec(node.right);
            print_tree_rec(node.left)
        },
        _ => {
            println!("{}", node.unwrap().value);
            return
        }
    }

}