// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // they said you can't construct a binary tree from preorder and postorder traversal only, and
        // they were right
        // but you can return any valid one, so that's ok for this problem
        let n = postorder.len();
        let mut post_val_to_idx = HashMap::new();
        for (idx, &node) in postorder.iter().enumerate() {
            post_val_to_idx.insert(node, idx);
        }

        build_tree(0, n - 1, 0, n - 1, &mut post_val_to_idx, &preorder)
    }
}

fn build_tree(
    i1: usize,
    i2: usize,
    j1: usize,
    j2: usize,
    post_val_to_idx: &mut HashMap<i32, usize>,
    preorder: &Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if i1 > i2 || j1 > j2 {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(preorder[i1])));

    // not only one node
    if i1 != i2 {
        let left_val = preorder[i1 + 1];
        let mid = post_val_to_idx[&left_val];

        let left_size = mid - j1 + 1;

        root.borrow_mut().left =
            build_tree(i1 + 1, i1 + left_size, j1, mid, post_val_to_idx, preorder);
        root.borrow_mut().right = build_tree(
            i1 + left_size + 1,
            i2,
            mid + 1,
            j2 - 1,
            post_val_to_idx,
            preorder,
        );
    }

    Some(root)
}
