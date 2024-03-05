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
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(root_node) = root {
        let node = root_node.borrow();
        if node.left.is_none() && node.right.is_none() && node.val == target_sum {
            return true;
        }
        return has_path_sum(node.left.clone(), target_sum - node.val)
            || has_path_sum(node.right.clone(), target_sum - node.val);
    }
    false
}
