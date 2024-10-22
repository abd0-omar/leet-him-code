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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        // straight-forward bfs with a binary heap, or an array that has the kth largest element
        // from a stream
        let mut q = std::collections::VecDeque::new();
        // only positive inputs
        let k = k as usize;
        let mut k_largest = vec![-1; k];
        let mut idx = 0usize;

        if let Some(node) = root {
            q.push_back(node);

            let mut _lvl = 0;
            while !q.is_empty() {
                let size = q.len();
                let mut cur_sum: Option<i64> = None;
                for _ in 0..size {
                    let cur = q.pop_front().unwrap();
                    if idx == 0 {
                        k_largest[idx] = cur.borrow().val as i64;
                        idx += 1;
                    }
                    let left = cur.borrow().left.clone();
                    if let Some(n) = left {
                        cur_sum = Some(cur_sum.unwrap_or(0) + n.borrow().val as i64);
                        q.push_back(n);
                    }
                    let right = cur.borrow().right.clone();
                    if let Some(n) = right {
                        cur_sum = Some(cur_sum.unwrap_or(0) + n.borrow().val as i64);
                        q.push_back(n);
                    }
                }
                if idx < k {
                    k_largest[idx] = cur_sum.unwrap_or(-1);
                    // sort by largest
                    idx += 1;
                } else if idx == k {
                    k_largest.sort_unstable_by(|a, b| b.cmp(a));
                    if cur_sum.unwrap_or(-1) > k_largest[k - 1] {
                        k_largest[k - 1] = cur_sum.unwrap_or(-1);
                        k_largest.sort_unstable_by(|a, b| b.cmp(a));
                    }
                } else {
                    if cur_sum.unwrap_or(-1) > k_largest[k - 1] {
                        k_largest[k - 1] = cur_sum.unwrap_or(-1);
                        k_largest.sort_unstable_by(|a, b| b.cmp(a));
                    }
                }
                _lvl += 1;
            }
        }
        k_largest[k - 1]
    }
}

#[allow(unused)]
struct Solution;
