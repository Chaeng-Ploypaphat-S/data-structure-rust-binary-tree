mod binary_tree;

use binary_tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();
    tree.add(10);
    tree.add(5);
    tree.add(15);
}