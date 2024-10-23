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
pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // bfs and vec, with level sum
    // then bfs again to populate the tree wit the sum
    let root = root?;
    let mut q = std::collections::VecDeque::new();
    let mut lvl_sum = vec![];

    q.push_back(root.clone());

    while !q.is_empty() {
        let size = q.len();
        let mut cur_sum = 0;
        for _ in 0..size {
            let node = q.pop_front().unwrap();
            cur_sum += node.borrow().val;
            if let Some(n) = node.borrow().left.clone() {
                q.push_back(n);
            }
            if let Some(n) = node.borrow().right.clone() {
                q.push_back(n);
            };
        }
        lvl_sum.push(cur_sum);
    }

    q.push_back(root.clone());
    root.borrow_mut().val = 0;
    let mut lvl_idx = 1;

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let node = q.pop_front().unwrap();
            //
            // let siblings_sum = match (node.borrow().left.clone(), node.borrow().right.clone()) {
            //     (None, None) => 0,
            //     (None, Some(r)) => r.borrow().val,
            //     (Some(l), None) => l.borrow().val,
            //     (Some(l), Some(r)) => l.borrow().val + r.borrow().val,
            // };

            let siblings_sum = {
                let left_val = node.borrow().left.as_ref().map_or(0, |n| n.borrow().val);
                let right_val = node.borrow().right.as_ref().map_or(0, |n| n.borrow().val);
                left_val + right_val
            };

            if let Some(left) = node.borrow().left.clone() {
                left.borrow_mut().val = lvl_sum[lvl_idx] - siblings_sum;
                q.push_back(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                right.borrow_mut().val = lvl_sum[lvl_idx] - siblings_sum;
                q.push_back(right);
            };

            // if let Some(n) = node.borrow().left.clone() {
            //     node.borrow().left.clone().unwrap().borrow_mut().val =
            //         lvl_sum[lvl_idx] - siblings_sum;
            //     q.push_back(n);
            // };
            //
            // if let Some(n) = node.borrow().right.clone() {
            //     node.borrow().right.clone().unwrap().borrow_mut().val =
            //         lvl_sum[lvl_idx] - siblings_sum;
            //     q.push_back(n);
            // };
        }
        lvl_idx += 1;
    }

    Some(root)
}
