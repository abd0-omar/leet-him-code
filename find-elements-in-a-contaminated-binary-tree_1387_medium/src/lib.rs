use std::{cell::RefCell, collections::HashSet, rc::Rc};

// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
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

struct FindElements {
    hs: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut hs = HashSet::new();
        if let Some(node) = root {
            Self::dfs(&mut hs, node, 0);
        }
        Self { hs }
    }

    fn dfs(hs: &mut HashSet<i32>, node: Rc<RefCell<TreeNode>>, value: i32) {
        let mut node = node.borrow_mut();
        node.val = value;
        hs.insert(value);

        if let Some(left) = node.left.clone() {
            Self::dfs(hs, left, 2 * value + 1);
        }
        if let Some(right) = node.right.clone() {
            Self::dfs(hs, right, 2 * value + 2);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.hs.contains(&target)
    }
}
