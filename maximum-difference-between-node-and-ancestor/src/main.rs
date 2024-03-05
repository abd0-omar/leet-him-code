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
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = 0;
    if let Some(node) = root.clone() {
        max = max.max(_max_ancestor_diff(Some(node.clone()), node.borrow().val));
        let node = node.borrow();
        if let Some(l) = node.left.clone() {
            let val = l.borrow().val;
            max = max.max(_max_ancestor_diff(Some(l), val));
        }
        if let Some(r) = node.right.clone() {
            let val = r.borrow().val;
            max = max.max(_max_ancestor_diff(Some(r), val));
        }
        // Self:: for leetcode
        // max = max.max(
        //     Self::max_ancestor_diff(node.left.clone())
        //         .max(Self::max_ancestor_diff(node.right.clone())),
        // );
    }
    max
}

// for root only
pub fn _max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>, root_val: i32) -> i32 {
    let mut max = 0;
    if let Some(node) = root {
        let node = node.borrow();
        if let Some(l) = node.left.clone() {
            max = max.max((l.borrow().val - root_val).abs());
            println!("l={:?}", l.borrow().val);
        }
        if let Some(r) = node.right.clone() {
            max = max.max((r.borrow().val - root_val).abs());
            println!("r={:?}", r.borrow().val);
        }
        max = max.max(
            _max_ancestor_diff(node.left.clone(), root_val)
                .max(_max_ancestor_diff(node.right.clone(), root_val)),
        );
    }
    max
}
