// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // already solved before, I thinkin my start days of doing leetcode
        // dailys
        // the obvious soultions is bfs like I did before
        // but to change it up a bit (and preserver my daily github and leetcode
        // streak, yeah idk what else to push to github during my exams and
        // college projects) I'll do dfs
        let mut result = Vec::new();
        dfs(root, 0, &mut result);
        result
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        // same as `if depth = result.len()`
        if let Some(cur_max) = result.get_mut(depth) {
            *cur_max = (*cur_max).max(node.val);
        } else {
            result.push(node.val);
        }
        if let Some(node) = node.left.clone() {
            dfs(Some(node), depth + 1, result);
        }
        if let Some(node) = node.right.clone() {
            dfs(Some(node), depth + 1, result);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works0() {
//         let root = vec![1,3,2,5,3,null,9];
//         let output = vec![1,3,9];
//         let result = Solution::tree node.
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }

//     #[test]
//     fn it_works1() {
//         let root = vec![1,2,3];
//         let output = vec![1,3];
//         let result = Solution::tree node.
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }
// }
