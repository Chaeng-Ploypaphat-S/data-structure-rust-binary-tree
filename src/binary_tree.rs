pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn add(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        if let Some(ref mut root) = self.root {
            Self::insert(root, new_node);
        } else {
            self.root = Some(new_node);
        }

        // Debugging
        println!("Added: {}", value);
        self.print_in_order();
    }

    fn insert(current: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < current.value {
            if let Some(ref mut left) = current.left {
                Self::insert(left, new_node);
            } else {
                current.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = current.right {
                Self::insert(right, new_node);
            } else {
                current.right = Some(new_node);
            }
        }
    }

    fn print_in_order(&self) {
        if let Some(ref root) = self.root {
            Self::in_order_traversal(root);
        }
    }

    fn in_order_traversal(node: &Box<Node>) {
        if let Some(ref left) = node.left {
            Self::in_order_traversal(left);
        }
        println!("{}", node.value);
        if let Some(ref right) = node.right {
            Self::in_order_traversal(right);
        }
    }
}