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


}