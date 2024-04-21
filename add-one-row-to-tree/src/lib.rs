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
pub fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
    depth: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut q = std::collections::VecDeque::new();
    if depth == 1 {
        let new_node = Rc::new(RefCell::new(TreeNode {
            val,
            left: root,
            right: None,
        }));
        return Some(new_node);
    }

    if let Some(node) = root.clone() {
        q.push_back(node);
    }

    let mut lvl = 1;
    let mut found_depth = false;
    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            // let cur = cur.borrow();

            if lvl == depth - 1 {
                found_depth = true;
                let left = cur.borrow().left.clone();
                let right = cur.borrow().right.clone();

                let new_node_left = Rc::new(RefCell::new(TreeNode {
                    val,
                    left,
                    right: None,
                }));
                let new_node_right = Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right,
                }));

                cur.borrow_mut().left = Some(new_node_left);
                cur.borrow_mut().right = Some(new_node_right);
            }

            if let Some(l) = cur.borrow().left.clone() {
                q.push_back(l);
            };

            if let Some(r) = cur.borrow().right.clone() {
                q.push_back(r);
            };
        }
        if found_depth {
            return root;
        }
        lvl += 1;
    }

    root
}
