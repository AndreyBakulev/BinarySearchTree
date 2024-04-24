use crate::node::Node;
use druid::widget::{Align, Flex, Label, Padding};
use druid::Widget;

pub(crate) struct Tree {
    root_node: Option<Box<Node>>,
}
impl Tree {
    pub fn new() -> Self {
        Tree { root_node: None }
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

    pub fn find(&mut self, value: i64,node: &Option<Box<Node>>) {
        if value == node.as_ref().unwrap().value{
            println!("Found {} in tree!",value);
            return
        }
        if value > node.as_ref().unwrap().value {
            match &node.as_ref().unwrap().right {
                Some(node) => self.find(value,&node.right),
                None => println!("{} not found in tree!",value),
            }
        } else {
            match &node.as_ref().unwrap().left {
                Some(node) => self.find(value,&node.left),
                None => println!("{} not found in tree!",value),
            }
        }
    }
    pub fn build_tree_widget(&self) -> impl Widget<()> {
        let mut widget = Flex::column();
        if let Some(root) = &self.root_node {
            widget.add_child(root.build_node_widget());
        }
        Align::centered(widget)
    }
}