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
pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    _find_bottom_left_value(root)
}

pub fn _find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::VecDeque;

    let mut q = VecDeque::new();

    if let Some(node) = root.clone() {
        q.push_back(node);
    }

    let mut rezult = root.unwrap().borrow().val;

    while !q.is_empty() {
        let size = q.len();
        for i in 0..size {
            let popped = q.pop_front().clone().unwrap();
            let popped_node = popped.borrow();

            if i == 0 {
                rezult = popped_node.val;
            }

            if let Some(left) = popped_node.left.clone() {
                q.push_back(left);
            };

            if let Some(right) = popped_node.right.clone() {
                q.push_back(right);
            };
        }
    }

    rezult
}
