fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => return true,

        (None, Some(_)) => {
            return false;
        }
        (Some(_), None) => {
            return false;
        }
        (Some(node1), Some(node2)) => {
            let node1 = node1.borrow();
            let node2 = node2.borrow();

            if node1.val != node2.val {
                return false;
            }

            return is_same_tree(node1.left.clone(), node2.left.clone())
                && is_same_tree(node1.right.clone(), node2.right.clone());
        }
    }
}
