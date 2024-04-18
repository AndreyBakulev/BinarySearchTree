pub(crate) struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            value,
            left,
            right,
        }
    }
}