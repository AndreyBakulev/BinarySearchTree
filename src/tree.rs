use crate::node::Node;
pub(crate) struct Tree{
    root_node: Node,
}
impl Tree{
    pub fn new(root_node: Node)-> Self{
        Tree{
            root_node
        }
    }

    pub fn insert(&mut self, value: i64) {
        let new_node = Box::new(Node::new(value, None, None));
        let mut current_node = &mut self.root_node;
        loop {
            if value < current_node.value {
                //checks if left is some Node
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
    pub fn print_tree(&self) {
        self.root_node.print_tree("", true);
    }
    //remove
}