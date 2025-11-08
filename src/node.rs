#[derive(Debug, Clone)]
pub enum Node {
    Number{value: i64},
    Add{lhs: Box<Node>, rhs: Box<Node>},
}

impl Node {
    pub fn add(lhs: Node, rhs: Node) -> Node {
        Node::Add{lhs: Box::new(lhs), rhs: Box::new(rhs)}
    }
}
