#![allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
}

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
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    _range_sum_bst(root, low, high)
}

pub fn _range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node) = root {
        let mut sum = 0;
        let node = node.borrow();
        if node.val >= low && node.val <= high {
            sum += node.val;
        }
        sum += _range_sum_bst(node.left.clone(), low, high);
        sum += _range_sum_bst(node.right.clone(), low, high);
        return sum;
    }
    0
}
