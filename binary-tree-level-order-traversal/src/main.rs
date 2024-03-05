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
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut rez = Vec::new();
    if let Some(node) = root {
        let mut q = VecDeque::new();
        q.push_back(node.clone());
        // let node = node.borrow();
        // rez.push(vec![node.borrow().val]);

        while !q.is_empty() {
            let size = q.len();
            let mut v = Vec::new();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                v.push(cur.borrow().val);

                if let Some(l) = cur.borrow_mut().left.take() {
                    q.push_back(l);
                }

                if let Some(r) = cur.borrow_mut().right.take() {
                    q.push_back(r);
                }

                println!("q={:?}", q);
            }
            rez.push(v);
        }
    }
    rez
}
