// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // the obvious solution is bfs
        // but I think it can be solved dfs string format sorted, idk tho
        let mut q = std::collections::VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        }
        let mut count = 0;
        while !q.is_empty() {
            let mut cur_level = Vec::new();
            let size = q.len();
            for _ in 0..size {
                let cur_node = q.pop_front().unwrap();
                cur_level.push(cur_node.borrow().val);
                if let Some(node) = cur_node.borrow().left.clone() {
                    q.push_back(node);
                }
                if let Some(node) = cur_node.borrow().right.clone() {
                    q.push_back(node);
                };
            }
            let mut sorted = cur_level.clone();
            sorted.sort_unstable();
            // the hashmap solution in the editorial is better
            let mut idx = 0;
            while idx < sorted.len() {
                if sorted[idx] != cur_level[idx] {
                    let mut new_idx = 0;
                    for j in 0..cur_level.len() {
                        if cur_level[j] == sorted[idx] {
                            new_idx = j;
                            break;
                        }
                    }
                    cur_level.swap(idx, new_idx);
                    count += 1;
                    continue;
                }
                idx += 1;
            }
            println!("{:?}", &cur_level);
        }
        count
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works0() {
//         let root = vec![1,4,3,7,6,8,5,null,null,null,null,9,null,10];
//         let output = 3;
//         let result = Solution::minimum_operations(root);
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }

//     #[test]
//     fn it_works1() {
//         let root = vec![1,3,2,7,6,5,4];
//         let output = 3;
//         let result = Solution::minimum_operations(root);
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }

//     #[test]
//     fn it_works2() {
//         let root = vec![1,2,3,4,5,6];
//         let output = 0;
//         let result = Solution::tree node.
// // #[derive(Debug,Eq);
//         assert_eq!(result, output);
//     }
// }
