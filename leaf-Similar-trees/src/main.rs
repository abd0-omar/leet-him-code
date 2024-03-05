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
pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let v1 = _leaf_similar(root1);
    let v2 = _leaf_similar(root2);
    v1 == v2
}

pub fn _leaf_similar(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vec = Vec::new();
    if let Some(node) = root {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            vec.push(node.val);
        }
        vec.extend_from_slice(&_leaf_similar(node.left.clone()));
        vec.extend_from_slice(&_leaf_similar(node.right.clone()));
        return vec;
    }
    vec
}
