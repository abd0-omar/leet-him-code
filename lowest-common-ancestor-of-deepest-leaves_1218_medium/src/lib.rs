// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(root).0
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    if let Some(node) = root {
        // postorder
        let (left, left_depth) = dfs(node.borrow().left.clone());
        let (right, right_depth) = dfs(node.borrow().right.clone());
        if left_depth == right_depth {
            return (Some(node), left_depth + 1);
        } else if left_depth > right_depth {
            return (left, left_depth + 1);
        } else {
            return (right, right_depth + 1);
        }
    }
    (None, 0)
}
