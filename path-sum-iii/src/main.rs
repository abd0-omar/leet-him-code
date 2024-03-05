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
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let mut count = 0;
    path_sum_wait(root, target_sum, &mut count);
    count
}

pub fn path_sum_wait(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, count: &mut i32) -> i32 {
    if let Some(node) = root.clone() {
        _path_sum(root, target_sum, count, 0);
        path_sum_wait(node.borrow().left.clone(), target_sum, count);
        path_sum_wait(node.borrow().right.clone(), target_sum, count);
        return *count;
    }
    0
}

fn _path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    count: &mut i32,
    mut curr_sum: i32,
) {
    //works on root only
    if let Some(node) = root {
        let node = node.borrow();
        println!("node={:?}", node.val);
        println!("target_sum={:?}", target_sum);

        curr_sum += node.val;

        if target_sum == curr_sum {
            *count += 1;
        }

        _path_sum(node.left.clone(), target_sum, count, curr_sum);
        _path_sum(node.right.clone(), target_sum, count, curr_sum);
    }
}
