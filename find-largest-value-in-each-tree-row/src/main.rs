fn main() {
    println!("Hello, world!");
    // Create the binary tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));

    // Call your function to get the result
    let result = largest_values(root);

    // Expected output
    let expected_output = vec![1, 3, 9];

    // Check if the result matches the expected output
    assert_eq!(result, expected_output);
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

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut queue = std::collections::VecDeque::new();
    let mut result = Vec::new();

    if let Some(root_node) = root.as_ref() {
        queue.push_back(root_node.clone());
    }

    while !queue.is_empty() {
        let size = queue.len();
        let mut max_of_level = i32::MIN;

        for _ in 0..size {
            if let Some(node) = queue.pop_front() {
                let val = node.borrow().val;
                max_of_level = max_of_level.max(val);

                if let Some(left) = node.borrow().left.as_ref() {
                    queue.push_back(left.clone());
                }

                if let Some(right) = node.borrow().right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
        }

        result.push(max_of_level);
    }

    result
}
