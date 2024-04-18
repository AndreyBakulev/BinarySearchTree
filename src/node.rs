pub(crate) struct Node {
    pub(crate) value: i64,
    pub(crate) left: Option<Box<Node>>,
    pub(crate) right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            value,
            left,
            right,
        }
    }
    pub fn print_tree(&self, prefix: &str, is_left: bool) {
        if is_left {
            println!("{}{}", prefix, "├── ");
        } else {
            println!("{}{}", prefix, "└── ");
        }
        println!("{}{}", prefix, self.value);

        let new_prefix = if is_left {
            format!("{}│   ", prefix)
        } else {
            format!("{}    ", prefix)
        };

        if let Some(ref left_node) = self.left {
            left_node.print_tree(&new_prefix, true);
        }
        if let Some(ref right_node) = self.right {
            right_node.print_tree(&new_prefix, false);
        }
    }
}