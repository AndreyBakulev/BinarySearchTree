use std::fmt;

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
}