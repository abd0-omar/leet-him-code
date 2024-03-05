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
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut vec = Vec::new();
    _path_sum(root, target_sum, &mut Vec::new(), &mut vec);
    vec
}

fn _path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    small_vec: &mut Vec<i32>,
    vec: &mut Vec<Vec<i32>>,
) {
    if let Some(root_node) = root {
        let node = root_node.borrow();
        small_vec.push(node.val);
        if node.left.is_none() && node.right.is_none() && target_sum == node.val {
            vec.push(small_vec.clone());
        }
        _path_sum(node.left.clone(), target_sum - node.val, small_vec, vec);
        _path_sum(node.right.clone(), target_sum - node.val, small_vec, vec);
        small_vec.pop();
    }
}
