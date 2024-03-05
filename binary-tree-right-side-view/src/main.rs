fn main() {
    println!("Hello, world!");
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
use std::collections::VecDeque;
use std::rc::Rc;
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    bfs(root)
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v = Vec::new();
    if let Some(node) = root {
        let mut q = VecDeque::new();
        let node_val = node.borrow().val;
        q.push_back(node);
        v.push(node_val);
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                if let Some(l) = cur.borrow().left.clone() {
                    q.push_back(l);
                };
                if let Some(r) = cur.borrow().right.clone() {
                    q.push_back(r);
                };
            }
            if let Some(last) = q.back() {
                v.push(last.borrow().val);
            }
        }
    }
    v
}
