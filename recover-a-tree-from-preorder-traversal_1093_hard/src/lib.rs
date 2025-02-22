// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
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
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut dashes = 0;
        let traversal: Vec<char> = traversal.chars().collect();
        let mut i = 0;
        let n = traversal.len();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        while i < n {
            if traversal[i] == '-' {
                i += 1;
                dashes += 1;
            } else {
                // we are in a possibly more than one digit number (at least one digit)
                let mut j = i;
                while j < n && traversal[j].is_digit(10) {
                    j += 1;
                }

                let num = *&traversal[i..j]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                let node = Rc::new(RefCell::new(TreeNode::new(num)));

                while stack.len() > dashes {
                    stack.pop();
                }

                if let Some(last_node) = stack.last() {
                    if last_node.borrow().left.is_none() {
                        last_node.borrow_mut().left = Some(node.clone());
                    } else {
                        last_node.borrow_mut().right = Some(node.clone());
                    }
                }

                stack.push(node);
                dashes = 0;
                i = j;
            }
        }
        Some(stack[0].clone())
    }
}
