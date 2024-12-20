// https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
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
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = std::collections::VecDeque::new();
        if let Some(node) = root.clone() {
            q.push_back(node);
        }

        let mut lvl = 0;
        while !q.is_empty() {
            let size = q.len();
            let mut cur_level_nodes = Vec::with_capacity(size);
            for _ in 0..size {
                let cur_node = q.pop_front().unwrap();
                cur_level_nodes.push(cur_node.clone());

                if let Some(node) = cur_node.borrow().left.clone() {
                    q.push_back(node.clone());
                };
                if let Some(node) = cur_node.borrow().right.clone() {
                    q.push_back(node);
                };
            }
            if lvl % 2 == 1 {
                let (mut l, mut r) = (0, cur_level_nodes.len() - 1);
                while l < r {
                    let temp = cur_level_nodes[l].borrow().val;
                    cur_level_nodes[l].borrow_mut().val = cur_level_nodes[r].borrow().val;
                    cur_level_nodes[r].borrow_mut().val = temp;
                    l += 1;
                    r -= 1;
                }
            }
            lvl += 1;
        }

        root
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works0() {
//         let root = vec![2,3,5,8,13,21,34];
//         let output = vec![2,5,3,8,13,21,34];
//         let result = Solution::tree node.
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }
//
//     #[test]
//     fn it_works1() {
//         let root = vec![7,13,11];
//         let output = vec![7,11,13];
//         let result = Solution::reverse_odd_levels(root)
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }
//
//     #[test]
//     fn it_works2() {
//         let root = vec![0,1,2,0,0,0,0,1,1,1,1,2,2,2,2];
//         let output = vec![0,2,1,0,0,0,0,2,2,2,2,1,1,1,1];
//         let result = Solution::reverse_odd_levels(root)
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }
// }
