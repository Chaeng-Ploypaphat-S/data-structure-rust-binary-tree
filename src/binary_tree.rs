pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}